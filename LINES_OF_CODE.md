# Protocol Metrics

## Lines of Code (Rust)

Breakdown of the SocketFi workspace by package.

| Package                | Files | Blank | Comments | Code |
| ---------------------- | ----: | ----: | -------: | ---: |
| access                 |     2 |    18 |      139 |   63 |
| factory                |     4 |    56 |      176 |  187 |
| fee_manager            |     6 |    70 |      127 |  279 |
| identity_registry      |     6 |    85 |      219 |  300 |
| shared                 |    12 |    92 |      415 |  451 |
| social_payments_router |     6 |   116 |      272 |  474 |
| upgrade                |     5 |    81 |      376 |  297 |
| wallet                 |     9 |   133 |      352 |  575 |

---

## Summary

- **Total Rust Files:** 50
- **Total Code Lines:** 2,626
- **Total Comments:** 2,076
- **Total Blank Lines:** 651

---

## Insights

- The **Wallet contract** is the largest component (575 LOC)
- The **Router** and **Shared modules** contain significant logic complexity
- High comment count indicates strong documentation coverage
- Modular structure keeps contracts well-separated and maintainable

---

## Notes

- Metrics generated using `cloc`
- Excludes `target` and `.git` directories
- Rust-only analysis

---

## Command Used

```bash
for dir in */; do
  cloc "$dir" --include-lang=Rust --exclude-dir=target,.git
done
```
