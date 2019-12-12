use crate::UseCaseError;

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

struct CreateMergeRequestUseCaseInteractor {

}

impl CreateMergeRequestUseCaseInteractor {
    fn new() -> CreateMergeRequestUseCaseInteractor {
        CreateMergeRequestUseCaseInteractor {}
    }
}

impl super::UseCase<CreateMergeRequest, ()>  for CreateMergeRequestUseCaseInteractor {
    fn exec(&self, param: CreateMergeRequest) -> Result<(), UseCaseError> {
        MergeRequest::new();
        Ok(())
    }
}

trait MergeRequestRepository {

}

#[cfg(test)]
mod tests {
    use super::super::UseCase;
    use super::*;

    #[test]
    fn test_new() {
        let use_case_interactor = CreateMergeRequestUseCaseInteractor::new();
        let merge_request = CreateMergeRequest {};
        use_case_interactor.exec(merge_request);
    }
}