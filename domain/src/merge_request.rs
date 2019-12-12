
struct MergeRequest {
    id: Option<i32>
}

impl MergeRequest {
    fn new() -> MergeRequest {
        MergeRequest {id: None}
    }
}

struct CreateMergeRequest {

}

trait MergeRequestUseCase {
    fn create_merge_request(&self, param: &CreateMergeRequest);
}


struct MergeRequestUseCaseInteractor {

}

impl MergeRequestUseCaseInteractor {
    fn new() -> MergeRequestUseCaseInteractor {
        MergeRequestUseCaseInteractor{}
    }
}

impl MergeRequestUseCase for MergeRequestUseCaseInteractor {
    fn create_merge_request(&self, param: &CreateMergeRequest) {
        MergeRequest::new();
    }
}

trait MergeRequestRepository {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let use_case_interactor = MergeRequestUseCaseInteractor::new();
        let merge_request = CreateMergeRequest {};
        use_case_interactor.create_merge_request(&merge_request);
    }
}