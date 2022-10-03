/**
 * Example:
 * var li = ListNode(5)
 * var v = li.`val`
 * Definition for singly-linked list.
 */

class ListNode(var `val`: Int) {
    var next: ListNode? = null
}

class Solution {
    fun mergeTwoLists(list1: ListNode?, list2: ListNode?): ListNode? {
    var listNode1: ListNode? = list1
    var listNode2: ListNode? = list2
    var curr = ListNode(-1)
    val currCopy: ListNode = curr

    if (listNode1 == null) return listNode2
    if (listNode2 == null) return listNode1

    while (listNode1 != null && listNode2 != null) {
      if (listNode1.`val` > listNode2.`val`) {
        curr.next = listNode2
        curr = curr.next!!
        listNode2 = listNode2.next
      } else {
        curr.next = listNode1
        curr = curr.next!!
        listNode1 = listNode1.next
      }
    }

    when {
      listNode1 == null -> curr.next = listNode2
      listNode2 == null -> curr.next = listNode1
    }

    return currCopy.next
  }
}
