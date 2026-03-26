import { PrismaClient } from "@prisma/client";
import { createServer, IncomingMessage, ServerResponse } from "http";
import Fastify from "fastify";
import { ApolloServer } from "@apollo/server";
import { ApolloServerPluginDrainHttpServer } from "@apollo/server/plugin/drainHttpServer";
import { makeExecutableSchema } from "@graphql-tools/schema";
import { WebSocketServer } from "ws";
import { useServer } from "graphql-ws/use/ws";
import { readFileSync } from "fs";
import { join } from "path";
import { HeaderMap } from "@apollo/server";
import { startIndexer } from "./indexer";
import { buildResolvers } from "./graphql";

const db = new PrismaClient();

// ── Helpers ──────────────────────────────────────────────────────────────────

function readBody(req: IncomingMessage): Promise<string> {
  return new Promise((resolve, reject) => {
    let body = "";
    req.on("data", (chunk) => (body += chunk));
    req.on("end", () => resolve(body));
    req.on("error", reject);
  });
}

async function main() {
  await db.$connect();

  // ── REST (Fastify) ─────────────────────────────────────────────────────────
  const fastify = Fastify({ logger: true });

  fastify.get<{ Params: { subject: string } }>(
    "/attestations/:subject",
    async (req) => {
      return db.attestation.findMany({
        where: { subject: req.params.subject },
        orderBy: { timestamp: "desc" },
      });
    }
  );

  fastify.get<{ Params: { issuer: string } }>(
    "/attestations/issuer/:issuer",
    async (req) => {
      return db.attestation.findMany({
        where: { issuer: req.params.issuer },
        orderBy: { timestamp: "desc" },
      });
    }
  );

  const REST_PORT = Number(process.env.PORT ?? 3000);
  await fastify.listen({ port: REST_PORT, host: "0.0.0.0" });

  // ── GraphQL (Apollo Server v5 + graphql-ws) ────────────────────────────────
  const typeDefs = readFileSync(join(__dirname, "schema.graphql"), "utf-8");
  const schema = makeExecutableSchema({
    typeDefs,
    resolvers: buildResolvers(db),
  });

  // Raw HTTP server shared by Apollo (HTTP) and graphql-ws (WebSocket)
  const httpServer = createServer(async (req: IncomingMessage, res: ServerResponse) => {
    if (req.url !== "/graphql") {
      res.writeHead(404);
      res.end("Not found");
      return;
    }

    const body = await readBody(req);
    const headers = new HeaderMap();
    for (const [key, value] of Object.entries(req.headers)) {
      if (value) headers.set(key, Array.isArray(value) ? value.join(", ") : value);
    }

    const result = await apollo.executeHTTPGraphQLRequest({
      httpGraphQLRequest: {
        method: req.method ?? "GET",
        headers,
        search: new URL(req.url ?? "/graphql", "http://localhost").search,
        body: body ? JSON.parse(body) : undefined,
      },
      context: async () => ({ db }),
    });

    res.writeHead(result.status ?? 200, Object.fromEntries(result.headers));

    if (result.body.kind === "complete") {
      res.end(result.body.string);
    } else {
      // chunked / multipart
      for await (const chunk of result.body.asyncIterator) {
        res.write(chunk);
      }
      res.end();
    }
  });

  // WebSocket server for subscriptions — same port, same server
  const wsServer = new WebSocketServer({ server: httpServer, path: "/graphql" });
  const serverCleanup = useServer({ schema }, wsServer);

  const apollo = new ApolloServer({
    schema,
    introspection: true, // enables Apollo Sandbox at /graphql in development
    plugins: [
      ApolloServerPluginDrainHttpServer({ httpServer }),
      {
        async serverWillStart() {
          return {
            async drainServer() {
              await serverCleanup.dispose();
            },
          };
        },
      },
    ],
  });

  await apollo.start();

  const GQL_PORT = Number(process.env.GQL_PORT ?? 4000);
  httpServer.listen(GQL_PORT, "0.0.0.0", () => {
    console.log(`GraphQL endpoint:   http://0.0.0.0:${GQL_PORT}/graphql`);
    console.log(`GraphQL Playground: http://localhost:${GQL_PORT}/graphql`);
    console.log(`Subscriptions:      ws://localhost:${GQL_PORT}/graphql`);
  });

  // ── Indexer ────────────────────────────────────────────────────────────────
  startIndexer(db).catch((err) => {
    console.error("Indexer error:", err);
    process.exit(1);
  });
}

main();
