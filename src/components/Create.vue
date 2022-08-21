<template>
    <a-modal v-model:visible="props.visible" title="New Project" @ok="onCreate" okText="Create" @cancel="return2Start">
        <a-form>
            <a-form-item label="Project name">
                <a-input v-model:value="createState.appName" />
            </a-form-item>
            <template v-if="appExists">
                <a-alert type="error" show-icon style="margin-bottom: 5px;" description="The project aleady exists!" />
            </template>

            <div class="ant-upload ant-upload-drag" @click="handDraggerClick('EDGES')" style="margin-bottom: 5px;"
                @mouseenter="onMouseEnter('EDGES')">
                <p class="ant-upload-drag-icon">
                    <inbox-outlined></inbox-outlined>
                </p>
                <p class="ant-upload-text">Click or drag file to this area</p>
                <template v-if="createState.edgeFileName === null">
                    <p class="ant-upload-hint">Support for a csv or parquet file.</p>
                </template>
                <template v-else>
                    <p>{{ createState.edgeFileName }}</p>
                </template>
            </div>
            <template v-if="createState.edgeErrorVisible">
                <a-alert type="error" show-icon style="margin-bottom: 5px;"
                    description="The file is not set up correctly!" />
            </template>
            <template v-else></template>
            <a-button type="primary" block style="margin-bottom: 24px;" @click="showDrawer(TableType.EDGES)">Advance
                Edges Settings
            </a-button>

            <a-form-item name="switch" label="Add vertices">
                <a-switch v-model:checked="createState.setVertices" />
            </a-form-item>
            <template v-if="createState.setVertices">
                <div class="ant-upload ant-upload-drag" @click="handDraggerClick('VERTICES')"
                    style="margin-bottom: 5px;" @mouseenter="onMouseEnter('VERTICES')">
                    <p class="ant-upload-drag-icon">
                        <inbox-outlined></inbox-outlined>
                    </p>
                    <p class="ant-upload-text">Click or drag file to this area</p>
                    <template v-if="createState.vertexFileName === null">
                        <p class="ant-upload-hint">Support for a csv or parquet file.</p>
                    </template>
                    <template v-else>
                        <p>{{ createState.vertexFileName }}</p>
                    </template>
                </div>
                <template v-if="createState.vertexErrorVisible">
                    <a-alert type="error" show-icon style="margin-bottom: 5px;"
                        description="The file is not set up correctly!" />
                </template>
                <template v-else></template>
                <a-button type="primary" block @click="showDrawer(TableType.VERTICES)">Advance Vertices Settings
                </a-button>
            </template>
            <a-alert v-else message="The vertex table is automatically generated from the endpoints of the edge table."
                type="info" />

        </a-form>

    </a-modal>

    <a-drawer :width="500" title="Advance Settings" placement="right" :visible="createState.drawerVisible"
        @close="onDrawerClose">

        <AdvanceSetting ref="advance" :params="params"></AdvanceSetting>

        <template #extra>
            <a-button style="margin-right: 8px" @click="onDrawerClose">Cancel</a-button>
            <a-button type="primary" @click="onDrawerSave">Save</a-button>
        </template>
    </a-drawer>


</template>
<script setup lang="ts">
import { onMounted, reactive, watch } from 'vue';
import type { UnwrapRef } from 'vue';
import { useRouter } from 'vue-router';
import { TABLE, WORKSPACE } from '../router/name';
import { InboxOutlined } from '@ant-design/icons-vue';
import { ref } from 'vue';
import { listen, Event as TauriEvent } from '@tauri-apps/api/event';
import { selectFile } from '../lib/utils';
import { computed, ComputedRef, Ref, toRef, toRefs, UnwrapNestedRefs } from '@vue/reactivity';
import AdvanceSetting from './AdvanceSetting.vue';
import { AdvanceParams, AdvanceResult, CsvOptions, csvSettings, getSchema, parse_options, Rename, RenameForm, supportFileTypes } from '../lib/setting';
import { resolve, appDir, join } from '@tauri-apps/api/path';
import { useStore } from 'vuex';
import { invoke } from '@tauri-apps/api';
import { notification } from 'ant-design-vue';
import { FileType, ProjectInfo, TableType } from '../lib/common';
import { cardGridProps } from 'ant-design-vue/lib/card/Grid';

const props = defineProps<{ visible: boolean }>();
const emit = defineEmits(['cancelCreate']);
const store = useStore();

const router = useRouter();

interface CreateState {
    appName: string,
    // edge
    edgeFileName: string | null,
    edgeErrorVisible: boolean,

    // vertex
    setVertices: boolean,
    vertexFileName: string | null,
    vertexErrorVisible: boolean,

    drawerVisible: boolean,
    drawerHolder: TableType,
    dropHolder: TableType
};

const createState: UnwrapRef<CreateState> = reactive({
    appName: 'untitled',
    edgeFileName: null,
    edgeErrorVisible: false,

    setVertices: false,
    vertexFileName: null,
    vertexErrorVisible: false,

    drawerVisible: false,

    drawerHolder: TableType.Unknown,
    dropHolder: TableType.Unknown
});

const appExists = ref(false);

async function project_exists(): Promise<boolean> {
    const path = await resolve(store.state.appConfig.projects_home, createState.appName);
    const exists: Promise<boolean> = invoke('path_exists', { path: path });
    return exists;
};

watch(
    () => createState.appName,
    async () => {
        if (createState.appName === '') {
            appExists.value = false;
        } else {
            appExists.value = await project_exists();
        };
    }
);

const edgeFileType: ComputedRef<FileType> = computed(() => {
    if (createState.edgeFileName === null) { return FileType.Unknown }
    else if (createState.edgeFileName.endsWith('csv')) { return FileType.CSV }
    else if (createState.edgeFileName.endsWith('parquet')) { return FileType.PARQUET }
    else if (createState.edgeFileName.endsWith('json')) { return FileType.JSON }
    else { return FileType.Unknown }
});

const vertexFileType: ComputedRef<FileType> = computed(() => {
    if (createState.vertexFileName === null) { return FileType.Unknown }
    else if (createState.vertexFileName.endsWith('csv')) { return FileType.CSV }
    else if (createState.vertexFileName.endsWith('parquet')) { return FileType.PARQUET }
    else if (createState.vertexFileName.endsWith('json')) { return FileType.JSON }
    else { return FileType.Unknown }
});

var advanceSetting = reactive<AdvanceResult>({ edges: { csv: null, renames: [] }, vertices: { csv: null, renames: [] } });

const params = computed(() => {
    if (createState.drawerHolder === TableType.EDGES) {
        return <AdvanceParams><unknown>{ fileType: edgeFileType.value, path: createState.edgeFileName, holder: TableType.EDGES, advanceReturn: advanceSetting.edges }
    } else {
        return <AdvanceParams><unknown>{ fileType: vertexFileType.value, path: createState.vertexFileName, holder: TableType.VERTICES, advanceReturn: advanceSetting.vertices }
    }
});

const advance = ref<{ csvOptions: CsvOptions, renameForm: UnwrapRef<RenameForm>, error: Ref<boolean> } | null>(null);



function setdefaultCreateState() {
    createState.appName = 'untitled';
    createState.edgeFileName = null;
    createState.edgeErrorVisible = false;
    createState.setVertices = false;
    createState.vertexFileName = null;
    createState.vertexErrorVisible = false;
    createState.drawerVisible = false;
    createState.drawerHolder = TableType.Unknown;
    createState.dropHolder = TableType.Unknown;
};

// drawer

function showDrawer(type: TableType) {
    if (type === TableType.EDGES) {
        if (supportFileTypes.has(edgeFileType.value as FileType)) {
            createState.drawerHolder = type;
            createState.drawerVisible = true;
        } else {
            createState.edgeErrorVisible = true;
        }
    } else if (type === TableType.VERTICES) {
        if (supportFileTypes.has(vertexFileType.value as FileType)) {
            createState.drawerVisible = true;
            createState.drawerHolder = type;
        } else {
            createState.vertexErrorVisible = true;
        }
    };
};

const onDrawerClose = () => {
    createState.drawerVisible = false;
    createState.drawerHolder = TableType.Unknown;
    if (advance.value !== null) {
        // console.log(advance.value?.csvOptions.value);
    }
};

const onDrawerSave = () => {
    const settingError = advance.value?.error;
    if (settingError === true) {
        notification['error']({
            message: 'Schema Error',
            description:
                'edges must has src and dst columns, vertices must have id column!',
        });
    } else {
        if (createState.drawerHolder === TableType.EDGES) {
            advanceSetting.edges.renames = <Rename[]>(advance.value?.renameForm.renames);
            if (edgeFileType.value === FileType.CSV) {
                advanceSetting.edges.csv = <CsvOptions>(advance.value?.csvOptions);
            }
        } else if (createState.drawerHolder === TableType.VERTICES) {
            advanceSetting.vertices.renames = <Rename[]>(advance.value?.renameForm.renames);
            if (vertexFileType.value === FileType.CSV) {
                advanceSetting.vertices.csv = <CsvOptions>(advance.value?.csvOptions);
            }
        };
        createState.drawerVisible = false;
        createState.drawerHolder = TableType.Unknown;
    }

};


// edges

listen('tauri://file-drop', async (event: TauriEvent<string[]>) => {
    setTimeout(() => {
        if (createState.dropHolder === TableType.EDGES) {
            console.log('EDGES DROP');
            createState.edgeFileName = event.payload[0];
            createState.edgeErrorVisible = false;
        } else if (createState.dropHolder === TableType.VERTICES) {
            console.log('VERTICES DROP');
            createState.vertexFileName = event.payload[0];
            createState.vertexErrorVisible = false;
        }
    }, 20);

});

function onMouseEnter(type: string) {
    if (type === 'EDGES') {
        createState.dropHolder = TableType.EDGES;
    } else if (type === 'VERTICES') {
        createState.dropHolder = TableType.VERTICES;
    }
}

async function handDraggerClick(type: string) {
    const file = await selectFile();
    if (type === 'EDGES') {
        if (file !== null) {
            createState.edgeErrorVisible = false;
        };
        createState.edgeFileName = file;
    } else if (type === 'VERTICES') {
        if (file !== null) {
            createState.vertexErrorVisible = false;
        };
        createState.vertexFileName = file;
    }
};


// vertices


function return2Start(e: MouseEvent) {
    emit('cancelCreate');
    setdefaultCreateState();
};

const onCreate = async (e: MouseEvent) => {
    if (appExists.value) {
        notification['error']({
            message: 'Project exists',
            description:
                'Project ' + createState.appName + ' has exists!',
        });
    } else if (createState.edgeErrorVisible || (createState.vertexErrorVisible && createState.setVertices)) {
        notification['error']({
            message: 'File error',
            description:
                'The file is not set up correctly!',
        });
    } else {
        var vertex_setting: object | null = null;
        if (createState.setVertices) {
            vertex_setting = {
                file_path: createState.vertexFileName,
                file_type: vertexFileType.value,
                advance_setting: {
                    csv: advanceSetting.vertices.csv,
                    renames: advanceSetting.vertices.renames.map((rename: Rename) => {
                        return { name: rename.name, re_type: rename.type, rename: rename.rename }
                    })
                }
            }
        } else {
            vertex_setting = null;
        };
        var args = {
            projectParam: {
                app_name: createState.appName,
                project_home: store.state.appConfig.projects_home,
                edge_setting: {
                    file_path: createState.edgeFileName,
                    file_type: edgeFileType.value,
                    advance_setting: {
                        csv: advanceSetting.edges.csv,
                        renames: advanceSetting.edges.renames.map((rename: Rename) => {
                            return { name: rename.name, re_type: rename.type, rename: rename.rename }
                        })
                    }
                },
                vertex_setting: vertex_setting
            }
        };

        var checkPassed = true;

        if (createState.edgeFileName === null || (createState.setVertices && createState.vertexFileName === null)) {
            notification['error']({
                message: 'Setting Error',
                description:
                    'data file not set!',
            });
        } else {
            // check edges
            if (advanceSetting.edges.renames.length === 0 && createState.edgeFileName !== null) {
                const schema = await getSchema(edgeFileType.value, createState.edgeFileName,
                    advanceSetting.edges.csv === null ? parse_options(csvSettings) : advanceSetting.edges.csv);
                const renames = new Set();
                for (let key in schema) {
                    renames.add(key);
                };
                if (!(renames.has('src') && renames.has('dst'))) {
                    notification['error']({
                        message: 'Schema Error',
                        description:
                            'edges must has src and dst columns!',
                    });
                    checkPassed = false;
                }
            };

            // check vertices
            if (createState.setVertices && advanceSetting.vertices.renames.length === 0 && createState.vertexFileName !== null) {
                const schema = await getSchema(vertexFileType.value, createState.vertexFileName,
                    advanceSetting.vertices.csv === null ? parse_options(csvSettings) : advanceSetting.vertices.csv);
                const renames = new Set();
                for (let key in schema) {
                    renames.add(key);
                };
                if (!(renames.has('id'))) {
                    notification['error']({
                        message: 'Schema Error',
                        description:
                            'vertices must have id column!',
                    });
                    checkPassed = false;
                }
            };

            if (checkPassed) {
                await invoke('new_project', args);
                const path = await join(store.state.appConfig.projects_home, createState.appName);
                store.commit('set_opened', <ProjectInfo>{ name: createState.appName, path: path });
                console.log(path);
                router.push(WORKSPACE);
            }
        }
    }

};


onMounted(async () => {
    const drager = <HTMLElement>document.getElementById('edges-dragger');
    appExists.value = await project_exists();
});

</script>

<style>
.collection-create-form_last-form-item {
    margin-bottom: 0;
}

.a-upload-dragger {
    margin-bottom: 5px;
}
</style>