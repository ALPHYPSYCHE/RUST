
fn main() {
    println!(" ");
    println!("Tutorial 19 - Smart Pointers / Box ");
    println!("-----------------------------------");

    // BOX stores data on the HEAP, instead of the Stack (LIFO)
    // HEAP: request a certain amount of space, and the OS is going to find that amount of space somewhere in memory and allow you to store that information

    let box_int1 = Box::new(19);
    println!("box_int1 = {}", box_int1);

    // Create a Binary tree
    // We can't include nodes in a node because the size of a node depends on the size of multiple nodes which confuses the compiler

    // this one will gives you an error:

    // struct TreeNode<T> {
    //     pub left_hand: TreeNode<T>,
    //     pub right_hand: TreeNode<T>,
    //     pub key: T,
    //     }
    

    // Rust doesn't like Null values, so we have to use Option.
    
    struct TreeNode<T> {
        pub left_hand: Option<Box<TreeNode<T>>>,
        pub right_hand: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left_hand: None,
                right_hand: None,
                key: key,
            }
        }

        pub fn left_hand(mut self, node: TreeNode<T>) -> Self {
            self.left_hand = Some(Box::new(node));
            self
        }

        pub fn right_hand(mut self, node: TreeNode<T>) -> Self {
            self.right_hand = Some(Box::new(node));
            self
        }
    }

    let root = TreeNode::new(1)
    .left_hand(TreeNode::new(2))
    .right_hand(TreeNode::new(3));
}
