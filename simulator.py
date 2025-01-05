# simulation/simulator.py
import heapq
from decimal import Decimal, getcontext

from models.jobs import AperiodicJob, PeriodicJob
from schedulers.scheduler import Scheduler

# 設定精度
getcontext().prec = 10  # 可根據需求調整精度


class Simulator:
    def __init__(self, scheduler: Scheduler, max_sim_time: int = 3000) -> None:
        self.scheduler = scheduler
        self.scheduler.simulator = self
        # 模擬參數
        self.max_sim_time = max_sim_time
        self.current_time = 0

        # 模擬指標
        self.miss_pjob_number = 0
        self.total_pjob_number = 0
        self.finished_p_job_number = 0
        self.finished_ajob_number = 0
        self.total_ajob_response_time = 0

        # 模擬數據
        self.simulation_data = []
        self.periodic_ready_queue: list[PeriodicJob] = []
        self.aperiodic_ready_queue: list[AperiodicJob] = []

    def simulate(self) -> None:
        while self.current_time <= self.max_sim_time:
            tick_data = {
                "time": self.current_time,
                "aperiodic_job_release": [],
                "periodic_release": [],
                "current_job": None,
            }

            # Check the feasibility of periodic jobs
            def increment_miss_job_number(
                x: int,
            ) -> None:
                self.miss_pjob_number += x

            self.scheduler.check_feasibility(
                self.current_time,
                increment_miss_job_number,
            )

            # Release new jobsa
            for job in self.scheduler.periodic_jobs:
                if self.current_time % job.period == 0:
                    new_job = job.release(self.current_time)
                    # Callback the job arrived event
                    self.scheduler.job_arrived(new_job)
                    heapq.heappush(self.periodic_ready_queue, new_job)
                    # Update the simulation indicators
                    self.total_pjob_number += 1
                    tick_data["periodic_release"].append(new_job.job_id)

            for job in self.scheduler.aperiodic_jobs:
                if job.arrival_time == self.current_time:
                    # Callback the job arrived event
                    self.scheduler.job_arrived(job)
                    heapq.heappush(self.aperiodic_ready_queue, job)
                    # Update the simulation indicators
                    tick_data["aperiodic_job_release"].append(job.job_id)

            # Sort the ready queue
            self.scheduler.sort_queue()

            # Select a job to execute
            selected_job = self.scheduler.select_job(self.current_time)
            tick_data["current_job"] = selected_job

            # Execute the selected job
            if selected_job:
                selected_job.remaining_time -= 1
                if selected_job.remaining_time == 0:
                    if isinstance(selected_job, AperiodicJob):
                        self.aperiodic_ready_queue.remove(selected_job)
                        # Update the simulation indicators
                        self.finished_ajob_number += 1
                        response_time = self.current_time - selected_job.arrival_time + 1
                        self.total_ajob_response_time += response_time
                    elif isinstance(selected_job, PeriodicJob):
                        self.periodic_ready_queue.remove(selected_job)
                        # Update the simulation indicators
                        self.finished_p_job_number += 1
                    # Callback the job completed event
                    self.scheduler.job_completed(selected_job)
            self.simulation_data.append(tick_data)

            # Clock tick +1
            self.current_time += 1

    def get_miss_rate(self) -> Decimal:
        if self.total_pjob_number == 0:
            return Decimal(0)
        return Decimal(self.miss_pjob_number) / Decimal(self.total_pjob_number)

    def get_average_response_time(self) -> Decimal:
        if self.finished_ajob_number == 0:
            return Decimal(0)
        return self.total_ajob_response_time / Decimal(self.finished_ajob_number)
