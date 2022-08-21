<template>
    <a-menu-item v-for="item in props.menu" :key="item.key">{{ item.name }}</a-menu-item>
</template>

<script setup lang="ts">
import { EditMenuItem } from '../../lib/workspace';

const props = defineProps<{ menu: EditMenuItem[] }>();

console.log(props.menu);



</script>