// This function takes two unsigned integers (`usize`)
// as inputs and returns their sum.
pub fn add(left: usize, right: usize) -> usize {
    // Return the sum of `left` and `right`
    left + right
}

// This module contains unit tests for the `add` function.
#[cfg(test)] // This annotation ensures that the test module is only 
// included when running tests.
mod tests {
    // Bring the `add` function from the parent scope into this module.
    use super::*;

    // This test checks if the `add` function works as expected.
    #[test] // The `#[test]` attribute marks this function as a test case.
    fn it_works() {
        // Call the `add` function with arguments 2 and 2, and store the result.
        let result = add(2, 2);
        // Assert that the result is equal to 4. If it is not, the test will fail.
        assert_eq!(result, 4);
    }
}