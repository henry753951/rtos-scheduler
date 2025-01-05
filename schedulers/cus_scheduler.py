# schedulers/cus_scheduler.py
import math
from typing import TYPE_CHECKING

from models.jobs import AperiodicJob, PeriodicJob
from schedulers.scheduler import Scheduler

if TYPE_CHECKING:
    from simulator import Simulator


class CUSScheduler(Scheduler):
    def __init__(self, server_size: float) -> None:
        super().__init__()
        self.server_size = server_size
        self.server_deadline = 0
        self.server_execution_time = 0
        self.simulator: Simulator = None

    def job_arrived(self, job: PeriodicJob | AperiodicJob) -> None:
        if isinstance(job, AperiodicJob):
            # R2 When the server is idle, and an aperiodic job arrives
            if self.simulator.aperiodic_ready_queue:
                return
            # R2.a
            if self.simulator.current_time < self.server_deadline:
                # do nothing
                pass
            # R2.b If the server is idle
            else:
                self.server_deadline = self.simulator.current_time + math.ceil(job.execution_time / self.server_size)
                self.server_execution_time = job.execution_time

    def job_completed(self, job: PeriodicJob | AperiodicJob) -> None:
        pass

    def deadline_arrived(self) -> None:
        # R3
        if self.simulator.aperiodic_ready_queue:
            next_job = self.simulator.aperiodic_ready_queue[0]
            self.server_deadline = self.simulator.current_time + math.ceil(next_job.execution_time / self.server_size)
            self.server_execution_time = next_job.execution_time

    def select_job(
        self,
        current_time: int,
    ) -> PeriodicJob | AperiodicJob | None:
        if current_time >= self.server_deadline:
            self.deadline_arrived()

        selected_job = self._choose_job(
            self.simulator.periodic_ready_queue[0] if self.simulator.periodic_ready_queue else None,
            self.simulator.aperiodic_ready_queue[0]
            if self.simulator.aperiodic_ready_queue and self.server_execution_time != 0
            else None,
        )

        if isinstance(selected_job, AperiodicJob):
            self.server_execution_time -= 1

        # Print the selected job
        # if isinstance(selected_job, PeriodicJob):
        #     print(
        #         f"😊 Executing periodic job {selected_job.job_id} at {self.simulator.current_time} with deadline {selected_job.absolute_deadline}",
        #     )
        # elif isinstance(selected_job, AperiodicJob):
        #     print(
        #         f"❤️ Executing aperiodic job {selected_job.job_id} at {self.simulator.current_time}",
        #     )
        #     if self.simulator.periodic_ready_queue:
        #         print(
        #             f"\t Server Deadline: {self.server_deadline}, Periodic Job {self.simulator.periodic_ready_queue[0].job_id } Deadline: {self.simulator.periodic_ready_queue[0].absolute_deadline}",
        #         )
        #     else:
        #         print(
        #             f"\t Server Deadline: {self.server_deadline}, No periodic job in the queue",
        #         )
        return selected_job

    def _choose_job(
        self,
        periodic_job: PeriodicJob,
        aperiodic_job: AperiodicJob,
    ) -> PeriodicJob | AperiodicJob | None:
        if periodic_job is None and aperiodic_job is None:
            return None
        if aperiodic_job is None:
            return periodic_job
        if periodic_job is None:
            return aperiodic_job

        if self.server_deadline > periodic_job.absolute_deadline:
            return periodic_job
        return aperiodic_job
