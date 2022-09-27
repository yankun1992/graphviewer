// Setting

import { invoke } from "@tauri-apps/api";
import { reactive } from "vue";
import { FileType, TableType } from "./common";

export interface AppConfig {
    projects_home: string,
    history_file: string
}


export async function get_config(): Promise<AppConfig> {
    return await invoke("read_app_config");
}

export interface AdvanceReturn {
    csv: CsvOptions | null,
    renames: Rename[]
}

export interface AdvanceResult {
    edges: AdvanceReturn,
    vertices: AdvanceReturn
}

export interface AdvanceParams {
    fileType: FileType,
    path: string,
    holder: TableType,
    advanceReturn: AdvanceReturn
}

export interface CsvOptions {
    has_header: boolean,
    ignore_parser_errors: boolean,
    delimiter: string,
    comment_char: string,
    quote_char: string
}

export interface CsvSettings {
    csvCheckbox: string[],
    delimiter: string,
    commentChar: string,
    quoteChar: string
}

export const csvSettings = reactive<CsvSettings>({
    csvCheckbox: ["0", "1"],
    delimiter: ",",
    commentChar: "",
    quoteChar: ""
});


export function parse_options(settings: CsvSettings): CsvOptions {
    return <CsvOptions>{
        has_header: settings.csvCheckbox.some(id => id === "0"),
        ignore_parser_errors: settings.csvCheckbox.some(id => id === "1"),
        delimiter: settings.delimiter,
        comment_char: settings.commentChar,
        quote_char: settings.quoteChar
    };
}

export const dataTypes = ["Boolean", "UInt8", "UInt16", "UInt32", "UInt64", "Int8", "Int16", "Int32", "Int64", "Float32", "Float64", "Utf8"];

export interface Column {
    name: string,
    type: string
}

export const columns = [
    {
        title: "Column Name",
        dataIndex: "name",
        key: "name"
    },
    {
        title: "Column Type",
        dataIndex: "type",
        key: "type"
    }
];

export interface Rename {
    name: string | null;
    type: string | null;
    rename: string | null;
}

export interface RenameForm {
    renames: Rename[];
}

export const supportFileTypes: Set<FileType> = new Set([FileType.CSV, FileType.PARQUET, FileType.JSON]);

export async function getSchema(fileType: FileType, path: string, options: CsvOptions | null): Promise<object> {
    switch (fileType) {
        case FileType.CSV: {
            const arg = { file: path, options: options };
            return <object>((await invoke("get_csv_schema", arg) as any)["inner"]);
        }

        case FileType.PARQUET: {
            return <object>((await invoke("get_parquet_schema", { file: path }) as any)["inner"]);
        }

        case FileType.JSON: {
            return <object>((await invoke("get_json_schema", { file: path }) as any)["inner"]);
        }

        case FileType.Unknown: {
            return {};
        }
    }
}