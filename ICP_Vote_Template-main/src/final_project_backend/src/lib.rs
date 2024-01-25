use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;   type alias 'Memory' is used #warn()

const MAX_VALUE_SIZE: u32 = 5000;   constant 'MAX_SIZE' is never used

#[derive(Debug, CandidType, Deserialize)]
3 implementations
enun Choice{
    Approve,
    Reject,
    pass,
}
#[derive(Debug, CandidType, Deserialize)] 
3 Implementations 
enun VoteError {

    AlreadyVoted,
    ProposalIsNotActive,
    NoSuchProposal,
    AccessRejected,
    UpdateError
}
You, 2 second ago | 1 author (You)
#[derive(Debug, CandidType, Deserialize)]
3 Implementations 
struct Proposal{

    description: String,
    approve: u32,
    reject: u32,
    pass: u32,
    is_active: bool,
    voted: Vec<candid::Principal>,
    owner: candid::Principal,
}
You, 2 second ago | 1 author (You)
#[derive(Debug, CandidType, Deserialize)]
3 Implementations 
struct CreateProposal {

    description: String,
    is_active: bool,
}

    
 impl Storable for Proposal {
     fn to_bytes(&self) -> Cow<[u8]> {
         Cow::Owned(Encode!(self).unwrap())
     }

     fn from_bytes(bytes: Cow<[u8]>) -> Self {
         Decode!(bytes.as_ref(), Self).unwrap()
     }
 }

 impl BoundedStorable for Proposal {
     const MAX_SIZE: u32 = MAX_VALUE_SIZE;
     const IS_FIXED_SIZE: bool = false;
 }

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    
    static PROPOSAL_MAP: RefCell<StableBTreeMap<u64,Proposal,Memory> = RefCell::new(StableBTreeMap::init:(DefaultMemoryImpl::init MEMORY_MANAGER.with(|m| m.borrow.get(MemoryId::new(0)))));


}

 #[ic_cdk::query]
 fn get_proposal(key: u64) -> Option<Proposal> {  
    PROPOSAL_MAP.with (|p: &RefCell<BTreeMap<u64,Proposal,->>| p.borrow().get(&key)) 
 }

 #[ic_cdk::query]
 fn get_proposal_count() -> u64 {     
    PROPOSAL_MAP.with (|p: &RefCell<BTreeMap<u64,Proposal, ->>| p.borrow().len())
 }

 #[ic_cdk::update]
 fn create_proposal(key: u64, proposal: CreateProposal) -> Option<Proposal> {         }
    let value: Proposal = Proposal{

        description: Proposal.description,
        approve: 0u32,
        reject: 0u32,
        pass: 0u32,
        is_active: proposal.is_active,
        voted: vec![],
        owner: is_cdk::caller()
    };
    PROPOSAL_MAP.with (|p: &RefCell<BTreeMap<u64,Proposal, ->>| p.borrow_mut(),insert(key,value))
}

 #[ic_cdk::update]
 fn edit_proposal(key: u64, proposal: CreateProposal) -> Result<(), VoteError> {      }
 PROPOSAL_MAP.with (|p: &RefCell<BTreeMap<u64,Proposal, ->>| p.borrow_mut(),insert(key,value))

 let old_proposal_opt = proposal;

 match old_proposal_opt {
    some(value: proposal),
    none => return Err(VoteError:NoSuchProposal),
 }
 if old_proposal.owner != ic_cdk::caller {
    return Err(VoteError::AccessRejected);

 }
 Let value proposal = proposal{
    description: proposal.description,
    approve: old_proposal.approve,
    reject: old_proposal.reject,
    pass: old_proposal.pass,
    is_active:proposal.is_active,
    voted: old_proposal.voted,
    owner: old_proposal
 };
let res: Option<proposal> =p.borrow_null().insert(key, value);
  
   match res {
    some(_) => Ok{()},
    None => Err{VoteError::UpdateError}
   }
};


 #[ic_cdk::update]
 fn end_proposal(key: u64) -> Result<(), VoteError> {}




 #[ic_cdk::update]
 fn vote(key: u64, choice: Choice) -> Result<(), VoteError> {}
