# utils/file_reader.py
from models.jobs import AperiodicJob, PeriodicJob


def read_periodic_jobs(filename: str) -> list[PeriodicJob]:
    periodic_jobs = []
    try:
        with open(filename) as f:
            job_id = 1
            for line in f:
                if line.strip():
                    period, exec_time = map(int, line.strip().split(","))
                    job = PeriodicJob(period, exec_time, job_id)
                    periodic_jobs.append(job)
                    job_id += 1
    except FileNotFoundError:
        print(f"檔案 {filename} 未找到。")
    return periodic_jobs


def read_aperiodic_jobs(filename: str) -> list[AperiodicJob]:
    aperiodic_jobs = []
    try:
        with open(filename) as f:
            job_id = 1
            for line in f:
                if line.strip():
                    arrival, exec_time = map(int, line.strip().split(","))
                    job = AperiodicJob(arrival, exec_time, job_id)
                    aperiodic_jobs.append(job)
                    job_id += 1
    except FileNotFoundError:
        print(f"檔案 {filename} 未找到。")
    return aperiodic_jobs
