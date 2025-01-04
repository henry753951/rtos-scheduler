# schedulers/scheduler.py

from typing import TYPE_CHECKING

from models.jobs import AperiodicJob, PeriodicJob
from utils.file_reader import read_aperiodic_jobs, read_periodic_jobs

if TYPE_CHECKING:
    from simulator import Simulator


class Scheduler:
    def __init__(self) -> None:
        self.simulator: Simulator = None

    def load_periodic_jobs(self, periodic_file: str) -> None:
        self.periodic_jobs = read_periodic_jobs(periodic_file)

    def load_aperiodic_jobs(self, aperiodic_file: str) -> None:
        self.aperiodic_jobs = read_aperiodic_jobs(aperiodic_file)

    def sort_queue(self) -> None:
        """
        排序週期性工作：
        1. 按照絕對截止時間升序。
        2. 當絕對截止時間相同時，按 Job ID 升序。
        """
        self.simulator.periodic_ready_queue.sort(key=lambda x: (x.absolute_deadline, x.job_id))

    def check_feasibility(
        self,
        current_time: int,
        increment_miss_job_number: callable,
    ) -> list[PeriodicJob]:
        feasible_jobs = []
        count_p = 0
        for job in self.simulator.periodic_ready_queue.copy():
            if (job.absolute_deadline - current_time - job.remaining_time) >= 0:
                feasible_jobs.append(job)
            else:
                self.simulator.periodic_ready_queue.remove(job)
                print(
                    f"週期性工作 {job.job_id} 無法在截止時間前完成。 Absolute Deadline: {job.absolute_deadline} Current Time: {current_time}",
                )
                count_p += 1

        increment_miss_job_number(count_p)
        return feasible_jobs

    def select_job(
        self,
        current_time: int,
    ) -> PeriodicJob | AperiodicJob | None:
        raise NotImplementedError

    def job_arrived(self, job):
        pass

    def job_completed(self, job):
        pass
