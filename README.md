# 編譯環境
```
- rustc 1.82.0 (f6e511eec 2024-10-15)
- stable-x86_64-pc-windows-gnu
```
# 輸入與輸出
### inputs/{test}.txt
輸入檔案  
```
0, 5, 2, 1    (phase time, period, relative deadline, execution time)
2, 3, 2, 1
```

### outputs/{SchedulerName}/
用 SchedulerName 分資料夾，txt 與 JSON 內容若為 `無法排程任務` 與 `[]` 則為無法排程，並且不會有圖片輸出。
- **TXT** : 每一個clock的執行任務輸出
- **JSON** : 包含 Dead 和 Arrival 時間點
- **PNG** : 甘特圖


# 執行
### 編譯運行
```sh
cargo run
```
### 直接執行(Win64)
```sh
./RTOSScheduler.exe
```







