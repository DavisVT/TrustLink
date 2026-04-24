use soroban_sdk::{symbol_short, Address, Env, String, Symbol};

use crate::types::{Attestation, IssuerTier};

// ---------------------------------------------------------------------------
// Event topic string constants — single source of truth for all topic names.
// Use these constants when filtering events off-chain or in tests.
// ---------------------------------------------------------------------------
pub const TOPIC_ADM_INIT: &str = "adm_init";
pub const TOPIC_CREATED: &str = "created";
pub const TOPIC_IMPORTED: &str = "imported";
pub const TOPIC_BRIDGED: &str = "bridged";
pub const TOPIC_REVOKED: &str = "revoked";
pub const TOPIC_RENEWED: &str = "renewed";
pub const TOPIC_UPDATED: &str = "updated";
pub const TOPIC_DEL_REQ: &str = "del_req";
pub const TOPIC_EXPIRED: &str = "expired";
pub const TOPIC_ISS_REG: &str = "iss_reg";
pub const TOPIC_ISS_TIER: &str = "iss_tier";
pub const TOPIC_ISS_REM: &str = "iss_rem";
pub const TOPIC_CLMTYPE: &str = "clmtype";
pub const TOPIC_MS_PROP: &str = "ms_prop";
pub const TOPIC_MS_SIGN: &str = "ms_sign";
pub const TOPIC_ADM_XFER: &str = "adm_xfer";
pub const TOPIC_ADM_ADD: &str = "adm_add";
pub const TOPIC_ADM_REM: &str = "adm_rem";
pub const TOPIC_MS_ACTV: &str = "ms_actv";
pub const TOPIC_ENDORSED: &str = "endorsed";
pub const TOPIC_EXP_HOOK: &str = "exp_hook";
pub const TOPIC_ATT_XFER: &str = "att_xfer";
pub const TOPIC_PAUSED: &str = "paused";
pub const TOPIC_UNPAUSED: &str = "unpaused";
pub const TOPIC_REQ: &str = "req";
pub const TOPIC_REQ_OK: &str = "req_ok";
pub const TOPIC_REQ_NO: &str = "req_no";
pub const TOPIC_DEL_CRTD: &str = "del_crtd";
pub const TOPIC_DEL_RVKD: &str = "del_rvkd";
pub const TOPIC_WL_ON: &str = "wl_on";
pub const TOPIC_WL_ADD: &str = "wl_add";
pub const TOPIC_WL_REM: &str = "wl_rem";

/// Helper: convert a topic constant to a Soroban `Symbol` at runtime.
/// Use this in tests and off-chain code to match against emitted events.
pub fn topic(env: &Env, name: &str) -> Symbol {
    Symbol::new(env, name)
}

pub struct Events;

impl Events {
    pub fn admin_initialized(env: &Env, admin: &Address, timestamp: u64) {
        // TOPIC_ADM_INIT
        env.events()
            .publish((symbol_short!("adm_init"),), (admin.clone(), timestamp));
    }

    pub fn attestation_created(env: &Env, attestation: &Attestation) {
        // TOPIC_CREATED
        env.events().publish(
            (symbol_short!("created"), attestation.subject.clone()),
            (
                attestation.id.clone(),
                attestation.issuer.clone(),
                attestation.claim_type.clone(),
                attestation.timestamp,
                attestation.metadata.clone(),
            ),
        );
    }

    pub fn attestation_imported(env: &Env, attestation: &Attestation) {
        // TOPIC_IMPORTED
        env.events().publish(
            (symbol_short!("imported"), attestation.subject.clone()),
            (
                attestation.id.clone(),
                attestation.issuer.clone(),
                attestation.claim_type.clone(),
                attestation.timestamp,
                attestation.expiration,
            ),
        );
    }

    pub fn attestation_bridged(env: &Env, attestation: &Attestation) {
        // TOPIC_BRIDGED
        env.events().publish(
            (symbol_short!("bridged"), attestation.subject.clone()),
            (
                attestation.id.clone(),
                attestation.issuer.clone(),
                attestation.claim_type.clone(),
                attestation
                    .source_chain
                    .clone()
                    .unwrap_or(String::from_str(env, "")),
                attestation
                    .source_tx
                    .clone()
                    .unwrap_or(String::from_str(env, "")),
            ),
        );
    }

    pub fn attestation_revoked(
        env: &Env,
        attestation_id: &String,
        issuer: &Address,
        reason: &Option<String>,
    ) {
        // TOPIC_REVOKED
        env.events().publish(
            (symbol_short!("revoked"), issuer.clone()),
            (attestation_id.clone(), reason.clone()),
        );
    }

    pub fn attestation_revoked_with_reason(
        env: &Env,
        attestation_id: &String,
        issuer: &Address,
        reason: &Option<String>,
    ) {
        // TOPIC_REVOKED
        env.events().publish(
            (symbol_short!("revoked"), issuer.clone()),
            (attestation_id.clone(), reason.clone()),
        );
    }

    pub fn attestation_renewed(
        env: &Env,
        attestation_id: &String,
        issuer: &Address,
        new_expiration: Option<u64>,
    ) {
        // TOPIC_RENEWED
        env.events().publish(
            (symbol_short!("renewed"), issuer.clone()),
            (attestation_id.clone(), new_expiration),
        );
    }

    pub fn attestation_updated(
        env: &Env,
        attestation_id: &String,
        issuer: &Address,
        new_expiration: Option<u64>,
    ) {
        // TOPIC_UPDATED
        env.events().publish(
            (symbol_short!("updated"), issuer.clone()),
            (attestation_id.clone(), new_expiration),
        );
    }

    pub fn deletion_requested(
        env: &Env,
        subject: &Address,
        attestation_id: &String,
        timestamp: u64,
    ) {
        // TOPIC_DEL_REQ
        env.events().publish(
            (symbol_short!("del_req"), subject.clone()),
            (attestation_id.clone(), timestamp),
        );
    }

    pub fn attestation_expired(env: &Env, attestation_id: &String, subject: &Address) {
        // TOPIC_EXPIRED
        env.events().publish(
            (symbol_short!("expired"), subject.clone()),
            attestation_id.clone(),
        );
    }

    pub fn issuer_registered(env: &Env, issuer: &Address, admin: &Address, timestamp: u64) {
        // TOPIC_ISS_REG
        env.events().publish(
            (symbol_short!("iss_reg"), issuer.clone()),
            (admin.clone(), timestamp),
        );
    }

    /// Emitted when an issuer's tier is set or updated by the admin.
    pub fn issuer_tier_updated(env: &Env, issuer: &Address, tier: &IssuerTier) {
        // TOPIC_ISS_TIER
        env.events()
            .publish((symbol_short!("iss_tier"), issuer.clone()), *tier);
    }

    pub fn issuer_removed(env: &Env, issuer: &Address, admin: &Address, timestamp: u64) {
        // TOPIC_ISS_REM
        env.events().publish(
            (symbol_short!("iss_rem"), issuer.clone()),
            (admin.clone(), timestamp),
        );
    }

    pub fn claim_type_registered(env: &Env, claim_type: &String, description: &String) {
        // TOPIC_CLMTYPE
        env.events().publish(
            (symbol_short!("clmtype"), claim_type.clone()),
            description.clone(),
        );
    }

    /// Emitted when a new multi-sig proposal is created.
    pub fn multisig_proposed(
        env: &Env,
        proposal_id: &String,
        proposer: &Address,
        subject: &Address,
        threshold: u32,
    ) {
        // TOPIC_MS_PROP
        env.events().publish(
            (symbol_short!("ms_prop"), subject.clone()),
            (proposal_id.clone(), proposer.clone(), threshold),
        );
    }

    /// Emitted when an issuer co-signs a multi-sig proposal.
    pub fn multisig_cosigned(
        env: &Env,
        proposal_id: &String,
        signer: &Address,
        signatures_so_far: u32,
        threshold: u32,
    ) {
        // TOPIC_MS_SIGN
        env.events().publish(
            (symbol_short!("ms_sign"), signer.clone()),
            (proposal_id.clone(), signatures_so_far, threshold),
        );
    }

    /// Emitted when admin rights are transferred to a new address.
    pub fn admin_transferred(env: &Env, old_admin: &Address, new_admin: &Address) {
        // TOPIC_ADM_XFER
        env.events().publish(
            (symbol_short!("adm_xfer"),),
            (old_admin.clone(), new_admin.clone()),
        );
    }

    /// Emitted when an admin adds a new admin to the council.
    pub fn admin_added(env: &Env, by_admin: &Address, new_admin: &Address, timestamp: u64) {
        // TOPIC_ADM_ADD
        env.events().publish(
            (symbol_short!("adm_add"), by_admin.clone()),
            (new_admin.clone(), timestamp),
        );
    }

    /// Emitted when an admin removes an admin from the council.
    pub fn admin_removed(
        env: &Env,
        by_admin: &Address,
        removed_admin: &Address,
        timestamp: u64,
    ) {
        // TOPIC_ADM_REM
        env.events().publish(
            (symbol_short!("adm_rem"), by_admin.clone()),
            (removed_admin.clone(), timestamp),
        );
    }

    /// Emitted when a multi-sig proposal reaches threshold and the attestation is activated.
    pub fn multisig_activated(env: &Env, proposal_id: &String, attestation_id: &String) {
        // TOPIC_MS_ACTV
        env.events().publish(
            (symbol_short!("ms_actv"),),
            (proposal_id.clone(), attestation_id.clone()),
        );
    }

    /// Emitted when a registered issuer endorses an existing attestation.
    pub fn attestation_endorsed(
        env: &Env,
        attestation_id: &String,
        endorser: &Address,
        timestamp: u64,
    ) {
        // TOPIC_ENDORSED
        env.events().publish(
            (symbol_short!("endorsed"), endorser.clone()),
            (attestation_id.clone(), timestamp),
        );
    }

    /// Emitted when an expiration hook is triggered for a subject's attestation.
    pub fn expiration_hook_triggered(
        env: &Env,
        subject: &Address,
        attestation_id: &String,
        expiration: u64,
    ) {
        // TOPIC_EXP_HOOK
        env.events().publish(
            (symbol_short!("exp_hook"), subject.clone()),
            (attestation_id.clone(), expiration),
        );
    }

    /// Emitted when admin transfers an attestation to a new issuer.
    pub fn attestation_transferred(
        env: &Env,
        attestation_id: &String,
        old_issuer: &Address,
        new_issuer: &Address,
    ) {
        // TOPIC_ATT_XFER
        env.events().publish(
            (symbol_short!("att_xfer"), old_issuer.clone()),
            (attestation_id.clone(), new_issuer.clone()),
        );
    }

    /// Emitted when the admin pauses the contract.
    pub fn contract_paused(env: &Env, admin: &Address, timestamp: u64) {
        // TOPIC_PAUSED
        env.events()
            .publish((symbol_short!("paused"),), (admin.clone(), timestamp));
    }

    /// Emitted when the admin unpauses the contract.
    pub fn contract_unpaused(env: &Env, admin: &Address, timestamp: u64) {
        // TOPIC_UNPAUSED
        env.events()
            .publish((symbol_short!("unpaused"),), (admin.clone(), timestamp));
    }

    /// Emitted when a subject submits an attestation request to an issuer.
    pub fn attestation_requested(
        env: &Env,
        request_id: &String,
        subject: &Address,
        issuer: &Address,
        claim_type: &String,
        expires_at: u64,
    ) {
        // TOPIC_REQ
        env.events().publish(
            (symbol_short!("req"), issuer.clone()),
            (
                request_id.clone(),
                subject.clone(),
                claim_type.clone(),
                expires_at,
            ),
        );
    }

    /// Emitted when an issuer fulfills an attestation request.
    pub fn request_fulfilled(
        env: &Env,
        request_id: &String,
        issuer: &Address,
        attestation_id: &String,
    ) {
        // TOPIC_REQ_OK
        env.events().publish(
            (symbol_short!("req_ok"), issuer.clone()),
            (request_id.clone(), attestation_id.clone()),
        );
    }

    /// Emitted when an issuer rejects an attestation request.
    pub fn request_rejected(
        env: &Env,
        request_id: &String,
        issuer: &Address,
        reason: &Option<String>,
    ) {
        // TOPIC_REQ_NO
        env.events().publish(
            (symbol_short!("req_no"), issuer.clone()),
            (request_id.clone(), reason.clone()),
        );
    }

    /// Emitted when issuer creates delegation to sub-issuer for claim_type.
    pub fn delegation_created(
        env: &Env,
        delegator: &Address,
        delegate: &Address,
        claim_type: &String,
        expiration: Option<u64>,
    ) {
        // TOPIC_DEL_CRTD
        env.events().publish(
            (symbol_short!("del_crtd"), delegator.clone()),
            (delegate.clone(), claim_type.clone(), expiration),
        );
    }

    /// Emitted when issuer revokes delegation.
    pub fn delegation_revoked(
        env: &Env,
        delegator: &Address,
        delegate: &Address,
        claim_type: &String,
    ) {
        // TOPIC_DEL_RVKD
        env.events().publish(
            (symbol_short!("del_rvkd"), delegator.clone()),
            (delegate.clone(), claim_type.clone()),
        );
    }

    pub fn whitelist_mode_enabled(env: &Env, issuer: &Address) {
        // TOPIC_WL_ON
        env.events()
            .publish((symbol_short!("wl_on"), issuer.clone()), ());
    }

    pub fn whitelist_updated(env: &Env, issuer: &Address, subject: &Address, added: bool) {
        // TOPIC_WL_ADD / TOPIC_WL_REM
        let sym: Symbol = if added {
            symbol_short!("wl_add")
        } else {
            symbol_short!("wl_rem")
        };
        env.events().publish((sym, issuer.clone()), subject.clone());
    }
}
