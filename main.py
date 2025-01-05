import logging
import os
from pathlib import Path

from schedulers.cus_scheduler import CUSScheduler
from schedulers.tbs_scheduler import TBSScheduler
from simulator import Simulator

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    handlers=[
        logging.StreamHandler(),
        logging.FileHandler("simulation.log", mode="w"),
    ],
)


def process_simulation(scheduler: CUSScheduler | TBSScheduler, input_path: str, output_path: str, method_name: str) -> dict:
    # 載入工作資料
    scheduler.load_periodic_jobs(Path(input_path) / "periodic.txt")
    scheduler.load_aperiodic_jobs(Path(input_path) / "aperiodic.txt")

    # 初始化模擬器並執行模擬
    simulator = Simulator(scheduler, max_sim_time=1000)
    simulator.simulate()

    # 輸出結果
    miss_rate = simulator.get_miss_rate()
    avg_response_time = simulator.get_average_response_time()

    Path.mkdir(output_path, parents=True, exist_ok=True)
    result_file = Path(output_path) / f"{method_name}.txt"
    test_file = Path(input_path).name
    with Path.open(result_file, "w", encoding="utf-8") as f:
        f.write(f"Test File: {test_file} Method: {method_name}\n")
        f.write(f"Miss Rate: {miss_rate:.10f}\n")
        f.write(f"Average Response Time: {avg_response_time:.10f}\n")
        f.write(f"Finsihed Aperiodic Jobs: {simulator.finished_a_job_number}\n")
        f.write(f"Total Periodic Jobs: {simulator.total_pjob_number}\n")
        f.write(f"Total Missed Periodic Jobs: {simulator.miss_pjob_number}\n")

    logging.info(f"Results for {method_name} saved to {result_file}")

    # viewer = SimulationViewer(simulator)
    # viewer.run()

    return {
        "miss_rate": miss_rate,
        "avg_response_time": avg_response_time,
        "total_aperiodic_response_time": simulator.total_ajob_response_time,
        "finished_a_job_number": simulator.finished_a_job_number,
        "total_periodic_jobs": simulator.total_pjob_number,
        "total_missed_periodic_jobs": simulator.miss_pjob_number,
    }


def main() -> None:
    input_root = "./inputs"
    output_root = "./outputs"

    result = {}
    for folder in os.listdir(input_root):
        input_path = Path(input_root) / folder
        if not Path.is_dir(input_path):
            continue

        output_path = Path(output_root) / folder
        includes = []
        if folder not in includes and len(includes) != 0:
            continue
        # 使用 CUS
        cus_scheduler = CUSScheduler(server_size=0.2)
        cus_result = process_simulation(cus_scheduler, input_path, output_path, "CUS")

        # 使用 TBS
        tbs_scheduler = TBSScheduler(server_size=0.2)
        tbs_result = process_simulation(tbs_scheduler, input_path, output_path, "TBS")

        result[folder] = {
            "CUS": cus_result,
            "TBS": tbs_result,
        }

    # 輸出結果
    for folder, data in result.items():
        print("=" * 50)
        print(f"# Results for {folder}")
        for method, method_data in data.items():
            print(f"## Method: {method}")
            for key, value in method_data.items():
                print(f"{key}: {value}")


if __name__ == "__main__":
    main()
