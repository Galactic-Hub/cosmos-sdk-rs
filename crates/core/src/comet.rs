//!Package comet defines the Comet Service interface and BlockInfo types which applications
//! should use in order to get access to the current block's evidence, validators hash, proposer address.
//!
//! This information is specific to Comet

pub mod service {

    //    // BlockInfoService is an interface that can be used to get information specific to Comet
    //    type BlockInfoService interface {
    // GetCometBlockInfo(context.Context) BlockInfo
    //    }
    pub trait BlockInfoService {
        // todo!()
        // fn get_comet_block_info(&self, ctx: Context) -> BlockInfo;
    }

    /// BlockInfo is the information comet provides apps in ABCI
    pub trait BlockInfo {
        /// Evidence misbehavior of the block
        fn get_evidence(&self) -> dyn EvidenceList;
        /// ValidatorsHash returns the hash of the validators
        /// For Comet, it is the hash of the next validator set
        fn get_validators_hash(&self) -> Vec<u8>;
        /// ProposerAddress returns the address of the block proposer
        fn get_proposer_address(&self) -> Vec<u8>;
        /// DecidedLastCommit returns the last commit info
        fn get_last_commit(&self) -> dyn CommitInfo;
    }

    /// MisbehaviorType is the type of misbehavior for a validator
    pub type MisbehaviorType = i32;

    pub const UNKNOWN: MisbehaviorType = 0;
    pub const DUPLICATE_VOTE: MisbehaviorType = 1;
    pub const LIGHT_CLIENT_ATTACK: MisbehaviorType = 2;

    /// Validator is the validator information of ABCI
    pub trait Validator {
        fn address(&self) -> Vec<u8>;
        fn power(&self) -> i64;
    }

    pub trait EvidenceList {
        fn len(&self) -> i32;
        fn is_empty(&self) -> bool;
        fn get(&self, i: i32) -> dyn Evidence;
    }

    /// Evidence is the misbehavior information of ABCI
    pub trait Evidence {
        fn type_(&self) -> MisbehaviorType;
        fn validator(&self) -> dyn Validator;
        fn height(&self) -> i64;
        fn time(&self) -> time::Time;
        fn total_voting_power(&self) -> i64;
    }

    /// CommitInfo is the commit information of ABCI
    pub trait CommitInfo {
        fn round(&self) -> i32;
        fn votes(&self) -> dyn VoteInfos;
    }

    /// VoteInfos is an interface to get specific votes in a efficient way
    pub trait VoteInfos {
        fn len(&self) -> i32;
        fn is_empty(&self) -> bool;
        fn get(&self, i: i32) -> dyn VoteInfo;
    }

    /// BlockIdFlag indicates which BlockID the signature is for
    type BlockIDFlag = i32;

    pub const BLOCK_ID_FLAG_UNKNOWN: BlockIDFlag = 0;
    /// BlockIDFlagAbsent - no vote was received from a validator.
    pub const BLOCK_ID_FLAG_ABSENT: BlockIDFlag = 1;
    /// BlockIDFlagCommit - voted for the Commit.BlockID.
    pub const BLOCK_ID_FLAG_COMMIT: BlockIDFlag = 2;
    /// BlockIDFlagNil - voted for nil.
    pub const BLOCK_ID_FLAG_NIL: BlockIDFlag = 3;

    /// VoteInfo is the vote information of ABCI
    pub trait VoteInfo {
        fn validator(&self) -> dyn Validator;
        fn get_block_id_flag(&self) -> BlockIDFlag;
    }
}
