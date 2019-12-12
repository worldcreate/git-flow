
struct MergeRequest {

}

trait MergeRequestUseCase {
    fn create_merge_request(&self, &param: MergeRequest);
}

struct MergeRequestUseCaseInteractor {

}

impl MergeRequestUseCaseInteractor {
    fn new() -> MergeRequestUseCaseInteractor {
        MergeRequestUseCaseInteractor{}
    }
}

impl MergeRequestUseCase for MergeRequestUseCaseInteractor {
    fn create_merge_request(&self, &param: MergeRequest) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::merge_request::{MergeRequestUseCaseInteractor, MergeRequestUseCase};

    #[test]
    fn test_new() {
        let useCaseInteractor = MergeRequestUseCaseInteractor::new()
        useCaseInteractor.create_merge_request();
    }
}