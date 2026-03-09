/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where
    T: PartialEq,
    T: PartialOrd,
    T: Copy,
{
    // 没必要进行排序
    if array.len() <= 1 {
        return;
    }

    // 两元素直接对比
    if array.len() == 2 {
        if array[0] > array[1] {
            array.swap(1, 0);
        }
        return;
    }

    // 插入排序
    if array.len() <= 16 {
        let mut i: isize = 1;
        let mut j: isize = 0;

        // 从第二个元素开始遍历
        while i < array.len() as isize {
            // 如果后一个元素比前一个元素小，那么后一个元素应该向前移动
            if array[i as usize] < array[(i - 1) as usize] {
                // 保存后一个元素，腾出位置
                let tmp = array[i as usize];
                // 将前一个元素向后移动
                array[i as usize] = array[(i - 1) as usize];
                // 用一个指针指向前一个位置
                j = i - 1;
                // 从刚刚的“前一个元素位置”反向遍历
                while j >= 0 {
                    // 如果当前值比保存的元素大，说明要向后移动位置
                    if array[j as usize] > tmp {
                        array[(j + 1) as usize] = array[j as usize];
                    } else {
                        // 否则就应该跳出，这个时候是最合适的插入点
                        break;
                    };
                    // 继续向前，直到找到插入点，或者是超出下限
                    j -= 1;
                }
                // 由于指针总是和实际位置前进一位，最坏的情况有可能是 -1
                // 这里需要补上
                array[(j + 1) as usize] = tmp;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
