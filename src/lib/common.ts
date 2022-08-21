import { AppConfig } from "./setting";

export enum FileType {
    JSON = "JSON",
    PARQUET = "PARQUET",
    CSV = "CSV",
    Unknown = "Unknown"
};


export enum TableType {
    VERTICES = "VERTICES",
    EDGES = "EDGES",
    Unknown = "Unknown"
};

export interface ProjectInfo { name: string, path: string };