<template>
    <div class="addBox">
        <div class="mask" @click="close"></div>
        <div class="a_win">
            <div class="a_title">新增会话</div>
            <a-input class="a_input" size="large" v-model:value="data.name" addon-before="会话名"/>
            <a-input class="a_input" size="large" v-model:value="data.prompt" addon-before="Prompt"/>
            <a-button @click="save" type="primary" :icon="h(CheckCircleOutlined)">保存</a-button>
        </div>
    </div>
</template>
<script setup>
import {defineEmits, h, ref} from "vue";
import {CheckCircleOutlined} from '@ant-design/icons-vue';

let emit = defineEmits(['save', "toggle"])
let data = ref({
    name: '',
    prompt: ''
});
const save = () => {
    emit("save", {
        name: data.value.name,
        prompt: data.value.prompt
    })
}
const close = (e) => {
    console.log(e)
    e.preventDefault;
    emit("toggle", false)

}
</script>
<style scoped lang="scss">
.addBox {
    .mask {
        width: 100%;
        height: 100%;
        top: 0;
        left: 0;
        background: rgba(0, 0, 0, 0.2);
        position: fixed;
        z-index: 100;
    }

    .a_win {
        z-index: 1001;
        width: 400px;
        height: 200px;
        background: rgba(255, 255, 255, 1);
        position: fixed;
        top: 40%;
        left: 50%;
        transform: translate(-50%, -50%);
        border-radius: 10px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-around;

        .a_title {
            height: 20px;
            line-height: 20px;
            width: 100%;
            text-align: center;
        }

        .a_input {
            width: 350px;
        }
    }
}

</style>