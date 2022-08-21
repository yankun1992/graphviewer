<template>
    <a-table :columns="columns" :data-source="data">
        <template #headerCell="{ column }">
            <template v-if="column.key === 'name'">
                <span>
                    <smile-outlined />
                    Name
                </span>
            </template>
        </template>

        <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'name'">
                <a>
                    {{ record.name }}
                </a>
            </template>
            <template v-else-if="column.key === 'tags'">
                <span>
                    <a-tag v-for="tag in record.tags" :key="tag"
                        :color="tag === 'loser' ? 'volcano' : tag.length > 5 ? 'geekblue' : 'green'">
                        {{ tag.toUpperCase() }}
                    </a-tag>
                </span>
            </template>
            <template v-else-if="column.key === 'action'">
                <span>
                    <a>Invite 一 {{ record.name }}</a>
                    <a-divider type="vertical" />
                    <a>Delete</a>
                    <a-divider type="vertical" />
                    <a class="ant-dropdown-link">
                        More actions
                        <down-outlined />
                    </a>
                </span>
            </template>
        </template>
    </a-table>
</template>

<script setup lang="ts">
import { SmileOutlined, DownOutlined } from '@ant-design/icons-vue';
import { PageType } from '../../lib/workspace';
import { useStore } from 'vuex';

const props = defineProps<{ pageType: PageType, columns: object[] | null, data: object[] | null }>();

const columns = props.columns;

const data = props.data;

</script>

<style>
.ant-table-thead>tr>th {
    padding: 8px 8px;
}

.ant-table-tbody>tr>td {
    padding: 4px 4px;
}
</style>