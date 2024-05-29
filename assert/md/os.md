OS设计实现
May 29,2024
清华春夏季开源操作系统夏令营

## CPU特权级 OS/User

- RISC-V 特权级

  | Level | Encoding | Name             | Abbreviation |
  | ----- | -------- | ---------------- | ------------ |
  | 0     | 00       | User/Application | U            |
  | 1     | 01       | Supervisor       | S            |
  | 2     | 10       | Reserved         |              |
  | 3     | 11       | Machine          | M            |

  相比x86 多一个M态
