import tkinter as tk

import matplotlib.pyplot as plt
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg

from models.jobs import AperiodicJob, PeriodicJob
from simulator import Simulator


class SimulationViewer:
    """模擬數據瀏覽器，使用 Tkinter 和 Matplotlib 顯示甘特圖"""

    def __init__(self, simulator: Simulator):
        self.simulator = simulator

        # 創建 Tkinter 主窗口
        self.root = tk.Tk()
        self.root.title("模擬甘特圖瀏覽器")

        # 創建 Matplotlib 圖形
        self.fig, self.ax = plt.subplots(figsize=(10, 6))
        self.ax.set_title("甘特圖")
        self.ax.set_xlabel("時間")
        self.ax.set_ylabel("工作")

        # 將 Matplotlib 圖形嵌入 Tkinter
        self.canvas = FigureCanvasTkAgg(self.fig, master=self.root)
        self.canvas_widget = self.canvas.get_tk_widget()
        self.canvas_widget.pack(fill=tk.BOTH, expand=True)

        # 創建滑條控制器
        self.scale = tk.Scale(
            self.root,
            from_=0,
            to=self.simulator.max_sim_time,
            orient="horizontal",
            command=self.update_display,
            label="時間 (t)",
        )
        self.scale.pack(fill="x")

        # 初始化顯示
        self.update_display(0)

    def update_display(self, t):
        """更新甘特圖顯示內容"""
        t = int(t)
        self.ax.clear()

        # 設置甘特圖標題和軸
        self.ax.set_title("Gantt Chart")
        self.ax.set_xlabel("Time")
        self.ax.set_ylabel("Task")

        time_range = 5
        self.ax.set_xlim(max(0, t - time_range), t + 1)

        # 定義工作行位置
        row_height = 1
        offset_y = 0.5
        task_y_positions = {}
        y_position = offset_y

        # 定義週期性工作和非週期性工作行
        all_jobs = {job.job_id for job in self.simulator.scheduler.periodic_jobs}
        all_jobs.add("Server")  # 加入 Server 行
        for job_id in all_jobs:
            label = f"T{job_id}" if job_id != "Server" else "Server"
            task_y_positions[label] = y_position
            y_position += row_height

        # 構建 `broken_barh` 的數據
        bars = {label: [] for label in task_y_positions}
        bar_labels = {label: [] for label in task_y_positions}  # 用於記錄每個 BAR 的文字標示
        for tick in range(max(0, t - time_range), t + 1):
            tick_data = self.simulator.simulation_data[tick]
            time = tick_data["time"]
            current_job = tick_data["current_job"]

            if isinstance(current_job, PeriodicJob):
                bars[f"T{current_job.job_id}"].append((time, 1))
                bar_labels[f"T{current_job.job_id}"].append(str(current_job.job_id))
            elif isinstance(current_job, AperiodicJob):
                bars["Server"].append((time, 1))
                bar_labels["Server"].append(f"Job {current_job.job_id}")
            elif current_job in [None, "Idle"]:
                bars["Server"].append((time, 1))
                bar_labels["Server"].append("Idle")

        # 繪製每個工作的甘特圖
        for label, intervals in bars.items():
            if intervals:
                y_pos = task_y_positions[label]
                self.ax.broken_barh(
                    intervals,
                    (y_pos - 0.25, 0.5),
                    facecolors="tab:orange" if "T" in label else "tab:green",
                )

                # 為每個 BAR 添加標示
                for (start, length), text in zip(intervals, bar_labels[label], strict=False):
                    self.ax.text(
                        start + length / 2,
                        y_pos,
                        text,
                        ha="center",
                        va="center",
                        fontsize=8,
                        color="white",
                    )

        self.canvas.draw()

    def run(self):
        """運行瀏覽器"""
        self.root.mainloop()
