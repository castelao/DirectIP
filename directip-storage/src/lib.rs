use directip;

// {ROOT_STORAGE}/{IMEI}/some_id
//
// ToDo next:
// - Think about what would be the id for a filesystem case, i.e. the filename.
//   - MO::CDR & MT:UUID are potential ids to be used in the filename for FileSystem.
//   - I think I used MOMSN/MTMSN
// - For InMemory, we just need the intermediate catalog. Which will also be required for
//   the file system.
// - Catalog should be a dynamic one, some function that would return this HashMap. Think
//   on how to retrieve data. For saving it doesn't make much difference since we won't expose
//   the internal structure.
//   Doesn't need to return a HasMap but the function could return the location for a given
//   identifier, and in a second step it can return the object itself.
// - Let's start with InMemory since it will be easier to write tests, and move to FileSystem
//   next, and finally S3 storage last.

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
