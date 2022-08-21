import { invoke } from "@tauri-apps/api";
import { join } from "@tauri-apps/api/path";
import { useStore } from "vuex";

export interface EditMenuItem {
    key: string
    name: string
};

export const tableMenu: EditMenuItem[] = [
    {
        key: 'open',
        name: 'Open'
    }, {
        key: 'setting',
        name: 'Setting'
    }
];


export const viewerMenu: EditMenuItem[] = [
    {
        key: 'create',
        name: 'Create new viewer'
    }
];

export const graphMenu: EditMenuItem[] = [
    {
        key: 'open',
        name: 'Open'
    },
    {
        key: 'setting',
        name: 'Setting'
    },
    {
        key: 'delete',
        name: 'Delete'
    },
    {
        key: 'rename',
        name: 'Rename'
    },
];

export enum PageType {
    VERTICES = "VERTICES",
    EDGES = "EDGES",
    GRAPH = "GRAPH",
    SETTING = 'SETTING'
};

interface Column {
    datatype: string,
    name: string,
    values: object[]
}

interface Table {
    columns: Column[]
};

export async function openTable(tablePath: string): Promise<object> {
    var table: Table = await invoke('read_parquet_command', { file: tablePath });
    const raw: Column[] = table['columns'];

    var values = {};
    var columns: object[] = [];
    var data: object[] = [];
    for (let column of raw) {
        columns.push({ title: column['name'], dataIndex: column['name'], key: column['name'] });
        values[column['name']] = column['values'];
    };
    const len = raw[0]['values'].length;

    for (var i = 0; i < len; i++) {
        var obj = {};
        for (let col of columns) {
            obj[col['title']] = values[col['title']][i];
        };
        data.push(obj);
    };

    return { columns, data };
};