<template>
    <div class="container">
        <div class="line">
            <a-input class="a_input" size="large" v-model:value="formState.apiKey" addon-before="ApiKey"/>
        </div>
        <div class="line">
            <a-button danger @click="clearDataClick">清除所有会话记录</a-button>
        </div>
        <div class="line">
            <a-button type="primary" :size="'middle'" @click="save">
                <template #icon>
                    <DownloadOutlined/>
                </template>
                保存
            </a-button>
        </div>
        <a-modal v-model:open="modal" ok-text="确认" cancel-text="取消" title="清除会话数据" @ok="confirmClear">
            <p>这将会清除所有的聊天会话和聊天记录。请确认操作</p>
        </a-modal>
    </div>
</template>
<script setup>
import {onMounted, ref} from 'vue';
import {DownloadOutlined} from '@ant-design/icons-vue';
import {invoke} from '@tauri-apps/api/tauri'
import {message} from 'ant-design-vue';
import {emit} from '@tauri-apps/api/event';

let modal = ref(false)
let formState = ref({
    apiKey: "sk-xxxxxxxxxxxxxxx",
})
invoke("get_config").then(config => {
    formState.value.apiKey = config.api_key
})
onMounted(() => {
    let server = localStorage.getItem("server");
    if (server != null) {
        formState.value = JSON.parse(server);
    }
})
const clearDataClick = () => {
    modal.value = true
}
const confirmClear = async () => {
    localStorage.clear();
    emit("clear_data")
    modal.value = false
}
const save = async () => {
    await invoke("set_config", {
        config: {
            api_key: formState.value.apiKey
        }
    });
    message.success('已保存', 1);
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