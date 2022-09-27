<template>
  <div style="padding-top: 8px; padding-left: 36px; padding-right: 36px;">
    <a-divider>Graph edges</a-divider>
    <p>select graph edges colunms</p>
    <a-select v-model:value="edgesColunm" mode="multiple" style="width: 100%; padding-bottom: 16px;"
              placeholder="Please select" :options="edgesOptions"/>
    <p>edges filter sql expression</p>
    <a-textarea v-model:value="edgesFiletrExpr" placeholder="sql expression" :rows="1"/>

    <a-divider>Graph vertices</a-divider>
    <p>select graph vertices colunms</p>
    <a-select v-model:value="verticesColunm" mode="multiple" style="width: 100%" placeholder="Please select"
              :options="verticesOptions"/>
    <p>vertices filter sql expression</p>
    <a-textarea v-model:value="verticesFiletrExpr" placeholder="sql expression" :rows="1"/>

    <a-divider>Graph</a-divider>
    <a-switch v-model:checked="threeD" checked-children="3D" un-checked-children="2D" style="width: 120px;"/>
    <a-switch v-model:checked="textNode" checked-children="Text as nodes" un-checked-children="Circle as nodes"
              style="width: 120px;"/>
    <a-switch v-model:checked="threeD" checked-children="3D" un-checked-children="2D" style="width: 100px;"/>
    <a-switch v-model:checked="threeD" checked-children="3D" un-checked-children="2D" style="width: 100px;"/>
    <a-switch v-model:checked="threeD" checked-children="3D" un-checked-children="2D" style="width: 100px;"/>


  </div>
</template>
<script setup lang="ts">
import type {UnwrapRef} from 'vue';
import {onMounted, reactive, ref, toRaw} from 'vue';
import {invoke} from '@tauri-apps/api';
import {useStore} from 'vuex';
import {join} from '@tauri-apps/api/path';

const store = useStore();

const edgesColunm = ref<string[]>([]);
const edgesOptions = ref<object[]>([]);
const edgesFiletrExpr = ref('');

const verticesColunm = ref<string[]>([]);
const verticesOptions = ref<object[]>([]);
const verticesFiletrExpr = ref('');

const threeD = ref(false);
const textNode = ref(false);

interface FormState {
  name: string;
  delivery: boolean;
  type: string[];
  resource: string;
  desc: string;
}

onMounted(async () => {
  const projectPath: string = store.state.openedProject.path;

  const edgesPath = await join(projectPath, 'edges');
  const edgesSchema: object = await invoke('get_parquet_schema', {file: edgesPath});
  for (let col in edgesSchema['inner']) {
    if (col !== 'src' && col !== 'dst') {
      edgesOptions.value.push({value: col});
    }
  }

  const verticesPath = await join(projectPath, 'vertices');
  const verticesSchema: object = await invoke('get_parquet_schema', {file: verticesPath});
  for (let col in verticesSchema['inner']) {
    if (col !== 'id') {
      verticesOptions.value.push({value: col});
    }
  }


});

const selectedKeys = ref<string[]>(['2']);

const formState: UnwrapRef<FormState> = reactive({
  name: '',
  delivery: false,
  type: [],
  resource: '',
  desc: '',
});
const onSubmit = () => {
  console.log('submit!', toRaw(formState));
};
const labelCol = {style: {width: '150px'}};
const wrapperCol = {span: 14};

</script>

