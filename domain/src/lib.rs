pub mod issue;
mod merge_request;

struct UseCaseError {

}

trait UseCase<E, R> {
    fn exec(&self, e: E) -> Result<R, UseCaseError>;
}
