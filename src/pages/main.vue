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
            <a-empty class="empty" v-if="data.msgList.length===0" description="没有消息记录~"/>
        </div>

        <div class="input_area">

            <a-button :disabled="data.sending" class="circle_btn" type="primary" shape="circle" size="large">
                <template #icon>
                    <UploadOutlined/>
                </template>
            </a-button>


            <a-textarea :disabled="data.sending" class="input_box" v-model:value="data.inputText"
                        @keyup.enter="quickSend"
                        placeholder="输入你的问题,使用Shift+Enter快速发送"
                        :auto-size="{ minRows: 2, maxRows: 5 }"/>


            <a-button @click="sendMsg" :disabled="data.sending" class="circle_btn" type="primary" shape="circle"
                      size="large">
                <template #icon>
                    <SendOutlined/>
                </template>
            </a-button>
        </div>
        <div class="left_panel">
            <div class="lp_box">
                <div class="menu_name">模型选择</div>
                <a-select class="model_select">
                    <a-select-option value="jack">Jack</a-select-option>
                </a-select>
                <div class="menu_name">记录</div>
                <div class="record_box">
                    <div v-for="record in data.recordList" @click="load_record(record)">{{ record.title }}</div>
                </div>
            </div>
            <div class="lp_arrow">👉</div>
        </div>
    </div>
</template>
<script setup>
import {ref, nextTick, onMounted} from 'vue';
import {UploadOutlined, SendOutlined} from '@ant-design/icons-vue';
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";

let data = ref({
    sending: false,
    msgList: [],
    recordList: [
        {
            title: "xxxxxxxxx",
        },
        {
            title: "xxxxxxxxx",
        },
        {
            title: "xxxxxxxxx",
        },
        {
            title: "xxxxxxxxx",
        }
    ],
    inputText: ""
});
onMounted(() => {
    toBottom();
});
const sendMsg = () => {
    const ques_id = new Date().getTime();
    let ques_content = data.value.inputText;
    data.value.msgList.push({
        id: ques_id,
        role: "user",
        avatar: "https://res.ztion.cn/imgs/cat.png",
        content: ques_content,
    });

    invoke("send_ques", {
        req: {
            ans_id: ques_id + 1,
            model_name: "qwen2-1.5b-instruct",
            msg_history: data.value.msgList
        }
    })

    data.value.msgList.push({
        id: ques_id + 1,
        role: "assistant",
        avatar: "https://res.ztion.cn/imgs/cat.png",
        content: "",
    });
    data.value.inputText = "";
    toBottom();
}
const load_record = (record) => {

}

const quickSend = e => {
    if (e.shiftKey === true && e.key === "Enter") {
        sendMsg();
    }
};

listen("msg_chunk", e => {
    let msg = e.payload;
    console.log("收到消息片段", msg);
    merge_msg_chunk(msg.msg_id, msg.chunk_content);
});

const merge_msg_chunk = (msg_id, content) => {
    data.value.msgList.forEach(msg => {
        if (msg.id === msg_id) {
            msg.content += content;
            toBottom();
        }
    })
}

const toggleDis = (state) => {
    data.value.sending = state;
}

const toBottom = () => {
    nextTick(() => {
        window.scrollTo(0, document.documentElement.scrollHeight);
    });
};
</script>
<style scoped lang="scss">
.container {
    width: 100%;
    height: 100%;
    background: #f7f8fc;
    overflow: hidden;


    .left_panel {
        width: 250px;
        height: 450px;
        position: fixed;
        top: 8%;
        left: 0;
        z-index: 100;
        transition: all 200ms ease-in;
        display: flex;
        align-items: center;


        &:hover {
            left: 0;
            transition: all 200ms ease-out;
        }

        .lp_box {
            width: 220px;
            height: 100%;
            background: rgba(255, 255, 255, 1);
            border-top-right-radius: 10px;
            border-bottom-right-radius: 10px;
            border: 1px solid #ccc;
            padding: 10px;
            display: flex;
            flex-direction: column;
            align-items: center;

            .menu_name {
                width: 100%;
                text-align: center;
                font-size: 12px;
                margin-top: 10px;
            }

            .model_select {
                width: 180px;
                margin-top: 15px;

                > div {
                    color: blue;
                }
            }

            .record_box {
                margin-top: 15px;
                width: 200px;
                height: 500px;
                border: 1px solid #7873f1;
                border-radius: 10px;
                overflow-y: scroll;
                padding: 5px;

                .choose {
                    background: #7873f1;
                }

                > div {
                    margin-top: 5px;
                    width: 100%;
                    height: 40px;
                    line-height: 40px;
                    text-align: center;
                    border: 2px solid #7873f1;
                    border-radius: 5px;

                    &:hover {
                        cursor: pointer;
                        background: #7873f1;
                    }
                }
            }
        }

        .lp_arrow {
            width: 30px;
            height: 80px;
            border-right: 1px solid #ccc;
            border-top: 1px solid #ccc;
            border-bottom: 1px solid #ccc;
            border-top-right-radius: 5px;
            border-bottom-right-radius: 5px;
            display: flex;
            align-items: center;
            background: rgba(255, 255, 255, 1);
        }
    }

    .msg_area {
        width: 100%;
        min-height: 500px;
        padding-bottom: 100px;
        overflow: hidden;

        .empty {
            position: fixed;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
        }

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