/*
 * Copyright 2022 Yan Kun
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import { invoke } from "@tauri-apps/api";

export interface EditMenuItem {
    key: string;
    name: string;
}

export const tableMenu: EditMenuItem[] = [
    {
        key: "open",
        name: "Open"
    }, {
        key: "setting",
        name: "Setting"
    }
];


export const viewerMenu: EditMenuItem[] = [
    {
        key: "create",
        name: "Create new viewer"
    }
];

export const graphMenu: EditMenuItem[] = [
    {
        key: "open",
        name: "Open"
    },
    {
        key: "setting",
        name: "Setting"
    },
    {
        key: "delete",
        name: "Delete"
    },
    {
        key: "rename",
        name: "Rename"
    }
];

export enum PageType {
    VERTICES = "VERTICES",
    EDGES = "EDGES",
    GRAPH = "GRAPH",
    SETTING = "SETTING"
}

interface Column {
    datatype: string,
    name: string,
    values: object[]
}

interface Table {
    columns: Column[];
}

export async function openTable(tablePath: string): Promise<object> {
    const table: Table = await invoke("read_parquet_command", { file: tablePath });
    const raw: Column[] = table["columns"];

    const values = {};
    const columns: object[] = [];
    const data: object[] = [];
    for (let column of raw) {
        columns.push({ title: column["name"], dataIndex: column["name"], key: column["name"] });
        values[column["name"]] = column["values"];
    }

    const len = raw[0]["values"].length;

    for (let i = 0; i < len; i++) {
        const obj = {};
        for (let col of columns) {
            obj[col["title"]] = values[col["title"]][i];
        }

        data.push(obj);
    }


    return { columns, data };
}