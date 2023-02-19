
export class SysMonitorData {
    sensors!:any
    load_avg!:number
}

export class CpuData {
    chip_name!:string
    physical_core_count!:number
    global_usage!:number
    cores!:CpuCoreData[];
}

export class CpuCoreData {
    usage!:number
    frequency!:number
}

export class ProcessData {
    memory!:number
    name!:string
    pid!:string
}

export class SensorData {
    label!:string
    temperature!:number
}

export class BatteryData {
    temperature!:number
    cycle_count!:number
    state!:number
    percentage!:number
    state_of_health!:string
}

export class MemoryData {
    total_memory!:number
    total_swap!:number
    used_memory!:number
    used_swap!:number
}