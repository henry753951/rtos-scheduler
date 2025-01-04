# models/jobs.py
import math


class Job:
    """基礎工作類別，包含共同屬性"""

    def __init__(self, arrival_time: int, execution_time: int, job_id: int) -> None:
        self.arrival_time = arrival_time  # 抵達時間
        self.execution_time = execution_time  # 執行時間
        self.remaining_time = execution_time  # 剩餘執行時間
        self.completion_time = None  # 完成時間
        self.job_id = job_id  # 工作ID，用於排序
        self.absolute_deadline = None  # 絕對截止時間

    def __lt__(self, other) -> bool:
        """定義小於運算子，根據絕對截止時間排序，若相同則根據Job ID排序"""
        if self.absolute_deadline == other.absolute_deadline:
            return self.job_id < other.job_id
        return self.absolute_deadline < other.absolute_deadline


class PeriodicJob(Job):
    """週期性工作類別，繼承自 Job"""

    def __init__(self, period: int, execution_time: int, job_id: int):
        super().__init__(arrival_time=0, execution_time=execution_time, job_id=job_id)
        self.period = period  # 任務週期

    def release(self, current_time: int):
        """釋放新的週期性工作"""
        new_job = PeriodicJob(self.period, self.execution_time, self.job_id)
        new_job.arrival_time = current_time
        new_job.absolute_deadline = current_time + self.period
        return new_job


class AperiodicJob(Job):
    """非週期性工作類別，繼承自 Job"""

    def __init__(self, arrival_time: int, execution_time: int, job_id: int):
        super().__init__(arrival_time, execution_time, job_id)
        self.absolute_deadline = math.inf  # 非週期性工作截止時間暫設為無限大
