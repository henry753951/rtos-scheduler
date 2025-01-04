# main.py
from schedulers.cus_scheduler import CUSScheduler
from simulator import Simulator
from viewer import SimulationViewer


def main() -> None:
    scheduler = CUSScheduler(server_size=0.2)

    # 載入工作資料
    scheduler.load_periodic_jobs("periodic.txt")
    scheduler.load_aperiodic_jobs("aperiodic.txt")

    # 初始化模擬器並執行模擬 CUS
    simulator = Simulator(scheduler)
    simulator.simulate()

    # 輸出 CUS 結果
    print("結果：")
    print(f"Miss Rate: {simulator.get_miss_rate():.10f}")
    print(f"Average Response Time: {simulator.get_average_response_time():.10f}")

    viewer = SimulationViewer(simulator)
    viewer.run()


if __name__ == "__main__":
    main()
