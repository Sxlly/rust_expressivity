// Shae Sullivan
// 20643731
// Programming Languages - Assignment
// 17/09/24


fn main() { //decalre main method
    

    fn product_of_four(a: i32, b: i32, c: i32, d: i32) -> i32 { //declare function that takes 4 signed 32 bit integers -> declare result returned will be 32 bit signed integer

        let result = a * b * c * d; //decalre variable result -> a*b*c*d parameters 
        return result; //return result 
    }

    product_of_four(31, 34, 65, 67); //call product_of_four function pass it -> 31,34,65,67

}
