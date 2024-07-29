<template>
    <div class="container">
        <div class="line">
            <a-input class="a_input" size="large" v-model:value="formState.apiKey" addon-before="ApiKey"/>
        </div>
        <div class="line">
            <a-button type="primary" :size="'middle'" @click="save">
                <template #icon>
                    <DownloadOutlined/>
                </template>
                保存
            </a-button>
        </div>
    </div>
</template>
<script setup>
import {onMounted, ref} from 'vue';
import {DownloadOutlined} from '@ant-design/icons-vue';
import {invoke} from '@tauri-apps/api/tauri'

let formState = ref({
    apiKey: "sk-xxxxxxxxxxxxxxx",
})

onMounted(() => {
    let server = localStorage.getItem("server");
    if (server != null) {
        formState.value = JSON.parse(server);
    }
})
const save = () => {

}
</script>
<style scoped lang="scss">
.container {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 40px 10px 10px;
    justify-content: space-between;
    height: 300px;

    .line {
        width: 90%;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
}
</style>