<template>
    <template v-if="schemaErrorVisable">
        <template v-if="props.params.holder === TableType.EDGES">
            <a-alert type="error" style="margin-bottom: 5px;" description="The column must have src and dst!" />
        </template>
        <template v-else>
            <a-alert type="error" style="margin-bottom: 5px;" description="The column must have id!" />
        </template>
    </template>
    <template v-if="props.params.fileType === (FileType.CSV as string)">
        <a-form>
            <a-row>
                <a-form-item label="CSV Options">
                    <a-checkbox-group v-model:value="csvSettings.csvCheckbox" @change="onOptionsChange">
                        <a-checkbox value="0" name="type">Has header</a-checkbox>
                        <a-checkbox value="1" name="type">Ignore parser errors</a-checkbox>
                    </a-checkbox-group>
                </a-form-item>
            </a-row>
            <a-row>
                <a-col style="margin-right: 15px;">
                    <a-form-item label="Delimiter" name="delimiter">
                        <a-input v-model:value="csvSettings.delimiter" style="width: 40px;" @change="onOptionsChange" />
                    </a-form-item>
                </a-col>
                <a-col style="margin-right: 15px;">
                    <a-form-item label="Comment Char" name="comment_char">
                        <a-input v-model:value="csvSettings.commentChar" style="width: 40px;"
                            @change="onOptionsChange" />
                    </a-form-item>
                </a-col>
                <a-col>
                    <a-form-item label="Quote Char" name="quote_char">
                        <a-input v-model:value="csvSettings.quoteChar" style="width: 40px;" @change="onOptionsChange" />
                    </a-form-item>
                </a-col>
            </a-row>
        </a-form>
    </template>

    <a-table :columns="columns" :data-source="data" :pagination="false" bordered></a-table>
    <a-divider>Rename or Retype</a-divider>
    <a-form>
        <a-form-item v-for="(rename, index) in renameForm.renames" :key="rename.name"
            :name="['domains', index, 'rename']">
            <a-input v-model:value="rename.rename" placeholder="please input new column name"
                style="width: 90%;margin-right: 15px;">
                <template #addonBefore>
                    <a-select v-model:value="rename.name" style="width: 100px" @change="onRenameChange(rename)">
                        <a-select-option v-for="name in names" :value="name"></a-select-option>
                    </a-select>
                </template>
                <template #addonAfter>
                    <a-select v-model:value="rename.type" style="width: 100px">
                        <a-select-option v-for="tp in dataTypes" :value="tp"></a-select-option>
                    </a-select>
                </template>
            </a-input>
            <MinusCircleOutlined v-if="renameForm.renames.length > 0" class="dynamic-delete-button"
                @click="onRenameRemove(index)" />
        </a-form-item>
        <a-form-item>
            <a-button type="dashed" style="width: 100%" @click="addDomain">
                <PlusOutlined />
                Add field
            </a-button>
        </a-form-item>
    </a-form>

</template>

<script setup lang="ts">
import { PlusOutlined, MinusCircleOutlined } from '@ant-design/icons-vue';
import { onMounted, ref, watch } from 'vue';
import { computed, ComputedRef, reactive, Ref, toRefs, UnwrapNestedRefs, UnwrapRef } from '@vue/reactivity';
import { AdvanceParams, parse_options, dataTypes, Column, columns, Rename, csvSettings, CsvOptions, RenameForm, getSchema } from '../lib/setting';
import { invoke } from '@tauri-apps/api/tauri';
import { FileType, TableType } from '../lib/common';

const props = defineProps<{
    params: AdvanceParams,
}>();

const schema = ref<object>({});

var error = ref(false);
const schemaErrorVisable = computed(() => {
    const renames = new Set(renameForm.renames.map(rename => rename.rename));
    for (let key in schema.value) {
        renames.add(key);
    };
    var e: boolean = false;
    if (props.params.holder === TableType.EDGES) {
        e = !(renames.has('src') && renames.has('dst'));
    } else if (props.params.holder === TableType.VERTICES) {
        e = !(renames.has('id'));
    } else {
        e = false;
    };
    error.value = e;
    return e;
});

const renameForm: UnwrapRef<RenameForm> = reactive<RenameForm>({
    renames: [],
});

const addDomain = () => {
    renameForm.renames.push({
        name: null,
        type: null,
        rename: null
    });
};

const data = computed(() => {
    var arr: Column[] = [];
    for (let key in schema.value) {
        arr.push({ name: key, type: schema.value[key] as string });
    };
    return arr;
});

const names = computed(() => {
    return data.value.map(item => item.name);
});

var csvOptions = parse_options(csvSettings);


async function onRenameChange(rename: Rename) {
    rename.type = schema.value[<string>rename.name]
};

function onRenameRemove(index: number) {
    renameForm.renames.splice(index, 1);
};

async function setSchema() {
    const sa = await getSchema(props.params.fileType, props.params.path, csvOptions);
    schema.value = sa;
}

async function onOptionsChange() {
    csvOptions = parse_options(csvSettings);
    await setSchema();
    const renames: Rename[] = [];
    for (let rename of renameForm.renames) {
        if (schema.value.hasOwnProperty(<string>rename.name)) {
            renames.push(rename);
        };
    };
    renameForm.renames = renames;
};

onMounted(async () => {
    await setSchema();
});

watch(
    () => props.params,
    async (param, preparam) => {
        if (param.path !== null) {
            await setSchema();
            renameForm.renames = [];
            csvOptions = props.params.advanceReturn.csv !== null ? props.params.advanceReturn.csv : parse_options(csvSettings);
            renameForm.renames = props.params.advanceReturn.renames;
        };
    },
    { deep: true, }
);


defineExpose<{ csvOptions: CsvOptions, renameForm: UnwrapRef<RenameForm>, error: Ref<boolean> }>({
    csvOptions,
    renameForm,
    error
});

</script>

<style>
.dynamic-delete-button {
    cursor: pointer;
    position: relative;
    top: 4px;
    font-size: 24px;
    color: #999;
    transition: all 0.3s;
}

.dynamic-delete-button:hover {
    color: #777;
}

.dynamic-delete-button[disabled] {
    cursor: not-allowed;
    opacity: 0.5;
}
</style>