<template>
    <div class="container" id="container">

        <div class="msg_area" id="msg_area">
            <div v-for="msg in data.msgList" :class="msg.role==='user'?'msg_card user_msg':'msg_card'">
                <div class="userinfo">
                    <a-avatar :size="60" :src="msg.avatar"/>
                </div>
                <div class="msg_content">
                    <v-md-preview :text="msg.content"></v-md-preview>
                </div>
            </div>
        </div>

        <div class="input_area">

            <a-button :disabled="data.sending" class="circle_btn" type="primary" shape="circle" size="large">
                <template #icon>
                    <UploadOutlined/>
                </template>
            </a-button>


            <a-textarea :disabled="data.sending" class="input_box" v-model:value="data.inputText"
                        placeholder="输入你的问题,使用Shift+Enter快速发送"
                        :auto-size="{ minRows: 2, maxRows: 5 }"/>


            <a-button @click="sendMsg" :disabled="data.sending" class="circle_btn" type="primary" shape="circle"
                      size="large">
                <template #icon>
                    <SendOutlined/>
                </template>
            </a-button>


        </div>
    </div>
</template>
<script setup>
import {ref, nextTick} from 'vue';
import {UploadOutlined, SendOutlined} from '@ant-design/icons-vue';

let data = ref({
    sending: false,
    msgList: [
        {
            role: "user",
            avatar: "https://res.ztion.cn/imgs/cat.png",
            content: `你知道Sakura吗`,
        },
        {
            role: "AI",
            avatar: "https://res.ztion.cn/imgs/cat.png",
            content: `### Sakura
用于生成美观的单个接口图片和或文本，识别Swagger注解
### 使用方法
在Controller方法上面右键，点击查看接口
### 效果`,
        }, {
            role: "AI",
            avatar: "https://res.ztion.cn/imgs/cat.png",
            content: `### Sakura
用于生成美观的单个接口图片和或文本，识别Swagger注解
### 使用方法
在Controller方法上面右键，点击查看接口
### 效果`,
        }, {
            role: "AI",
            avatar: "https://res.ztion.cn/imgs/cat.png",
            content: `### Sakura
用于生成美观的单个接口图片和或文本，识别Swagger注解
### 使用方法
在Controller方法上面右键，点击查看接口
### 效果`,
        }, {
            role: "AI",
            avatar: "https://res.ztion.cn/imgs/cat.png",
            content: `### Sakura
用于生成美观的单个接口图片和或文本，识别Swagger注解
### 使用方法
在Controller方法上面右键，点击查看接口
### 效果`,
        },
    ],
    inputText: ""
});
const sendMsg = () => {
    data.value.msgList.push({
        role: "user",
        avatar: "https://res.ztion.cn/imgs/cat.png",
        content: data.value.inputText,
    });
    data.value.inputText = "";
    toBottom();
}

const toggleDis = (state) => {
    data.value.sending = state;
}

const toBottom = () => {
    nextTick(() => {
        let container = document.getElementById("msg_area");
        console.log(container)
        container.scrollTop = container.scrollHeight;
    });
};
</script>
<style scoped lang="scss">
.container {
    width: 100%;
    height: 100%;
    background: #f7f8fc;
    overflow: hidden;

    .msg_area {
        width: 100%;
        height: 100%;
        padding-bottom: 100px;
        overflow: hidden;


        .user_msg {
            flex-direction: row-reverse;
            background: white;
        }

        .msg_card {
            border-bottom: 1px solid #ccc;
            width: 100%;
            min-height: 100px;
            display: flex;


            .userinfo {
                height: 100%;
                width: 10%;
                display: flex;
                justify-content: center;
                padding-top: 15px;
                user-select: none;
            }

            .msg_content {
                height: 100%;
                width: 90%;
            }

        }
    }

    .input_area {
        width: 100%;
        height: 80px;
        position: fixed;
        left: 0;
        bottom: 0;
        display: flex;
        justify-content: space-evenly;
        align-items: center;
        z-index: 100;
        opacity: 1;
        background-color: rgba(255, 255, 255, 0.5);
        padding-bottom: 10px;

        .circle_btn {
            width: 50px;
            height: 50px;
        }

        .input_box {
            width: 80%;
        }

    }
}
</style>