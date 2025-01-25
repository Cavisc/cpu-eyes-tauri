import {
  PiSpeedometerBold,
  PiThermometerBold,
  PiClockBold,
  PiMemoryBold,
} from "react-icons/pi";

export default function Item({ item, value }) {
  return (
    <div data-tauri-drag-region className="flex flex-col items-center h-full gap-2 select-none">
      <span data-tauri-drag-region className="text-xl w-full text-center select-none">{value}</span>
      {item === "usage" && <PiSpeedometerBold data-tauri-drag-region className="text-2xl select-none" />}
      {item === "temp" && <PiThermometerBold data-tauri-drag-region className="text-2xl select-none" />}
      {item === "clock" && <PiClockBold data-tauri-drag-region className="text-2xl select-none" />}
      {item === "ram" && <PiMemoryBold data-tauri-drag-region className="text-2xl select-none" />}
    </div>
  );
}
