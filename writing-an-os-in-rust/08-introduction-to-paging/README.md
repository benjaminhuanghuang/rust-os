# Introduction to Paging

https://os.phil-opp.com/paging-introduction/

OS 要保证进程的内存空间隔离

## Segmentation

Segmentation uses an individual segment selector register for each active memory region

- Protect mode

  segment descriptors contain an index into a local or global descriptor table, which contains – in addition to an offset address – the segment size and access permissions

  By modifying the memory addresses before the actual access, segmentation already employed a technique: virtual memory.

- virtual memory

  virtual memory 要被 translation function 翻译成 physical address

- Segmentation 技术的死穴: fragmentation problem

  x86 64-bit mode 不再支持 Segmentation

## Paging

Paging uses a table structure called page table to store the mapping information.

- Page Tables

  A pointer to the currently active table is stored in a special CPU register. On x86, this register is called CR3.

- Multilevel Page Tables

## Paging on x86_64

The x86_64 architecture uses a 4-level page table and a page size of 4K
512 entries x 8B = 4K
![](./page-table-entry.png)

![](./x86_64-page-table-translation.png)

![](./virtual-address.png)

![](./x86_64-page-table-translation-steps.png)
