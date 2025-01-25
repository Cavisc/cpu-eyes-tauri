import useGetCpuFrequency from "../../hooks/useGetCpuFrequency";
import useGetCpuTemp from "../../hooks/useGetCpuTemp";
import useGetCpuUsage from "../../hooks/useGetCpuUsage";
import useGetMemoryUsage from "../../hooks/useGetMemoryUsage";

import Item from "../Item";

export default function Content() {
  const cpuUsage = useGetCpuUsage();
  const cpuTemp = useGetCpuTemp();
  const cpuFrequency = useGetCpuFrequency();
  const memoryUsage = useGetMemoryUsage();

  return (
    <div data-tauri-drag-region className="grid grid-flow-col grid-cols-20/20/30/30 gap-2 content-end h-full select-none">
      <Item item={"usage"} value={cpuUsage} />
      <Item item={"temp"} value={cpuTemp} />
      <Item item={"clock"} value={cpuFrequency} />
      <Item item={"ram"} value={memoryUsage} />
    </div>
  );
}
