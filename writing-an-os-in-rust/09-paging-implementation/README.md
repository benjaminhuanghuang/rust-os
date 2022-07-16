# Paging Implementation

https://os.phil-opp.com/paging-implementation/

## Accessing Page Tables

There are different ways to create these mappings that all allow us to access arbitrary page table frames

- Identity Mapping

  会产生 segmentation.

  虚拟地址空间也会被消耗

- Map at a Fixed Offset

  和 Identity Mapping 有同样的问题

- Map the Complete Physical Memory

- Temporary Mapping

  map the page tables frames only temporarily when we need to access them

- Recursive Page Tables
