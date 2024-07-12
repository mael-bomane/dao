use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Protocol Name Empty.")]
    ProtocolNameEmpty,
    #[msg("Protocol Name Too Long, 50 Characters Maximum.")]
    ProtocolNameTooLong,
    #[msg("Hacker Name Empty.")]
    HackerNameEmpty,
    #[msg("Hacker Name Too Long, 50 Characters Maximum.")]
    HackerNameTooLong,
    #[msg("Message empty.")]
    MessageEmpty,
    #[msg("Wrong Hack ID.")]
    WrongHackID,
    #[msg("Signer Not Protocol Owner.")]
    SignerNotProtocolOwner,
    #[msg("Signer Not Program Upgrade Authority.")]
    SignerNotProgramUpgradeAuthority,
    #[msg("This Protocol Not Program Upgrade Authority.")]
    ProtocolNotProgramUpgradeAuthority,
    #[msg("Program Already In Protocol List.")]
    ProgramAlreadyInProtocolList,
    #[msg("Program Not In Protocol List.")]
    ProgramNotInProtocolList,
    #[msg("Wrong Program ID.")]
    WrongProgramID,
    #[msg("Mismatch Protocol Owner And Program Owner.")]
    MismatchProtocolOwnerAndProgramOwner,
    #[msg("No Mint Provided.")]
    NoMintProvided,
}
