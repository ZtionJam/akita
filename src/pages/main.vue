<template>
    <div class="container" id="container">

        <div class="msg_area" id="msg_area">
            <div v-for="msg in data.msgList" :class="msg.role==='user'?'msg_card user_msg':'msg_card'">
                <div class="userinfo">
                    <a-avatar :size="60" :src="msg.avatar" :title="msg.role"/>
                </div>
                <div class="msg_content">
                    <v-md-preview :text="msg.content"></v-md-preview>
                </div>
            </div>
            <a-empty class="empty" v-if="data.msgList.length===0" description="没有消息记录~"/>
        </div>

        <div class="input_area">

            <a-button :disabled="data.sending" title="清除本会话" @click="clearChat" class="circle_btn" type="primary"
                      shape="circle"
                      size="large">
                <template #icon>
                    <DeleteOutlined/>
                </template>
            </a-button>


            <a-textarea :disabled="data.sending" class="input_box" v-model:value="data.inputText"
                        @keyup.enter="quickSend"
                        placeholder="输入你的问题,使用Shift+Enter快速发送"
                        :auto-size="{ minRows: 2, maxRows: 5 }"/>


            <a-button @click="sendMsg" title="发送" :disabled="data.sending" class="circle_btn" type="primary"
                      shape="circle"
                      size="large">
                <template #icon>
                    <SendOutlined/>
                </template>
            </a-button>
        </div>
        <div class="left_panel">
            <div class="lp_box">
                <div class="menu_name">模型选择</div>
                <a-select class="model_select" v-model:value="data.nowModelName">
                    <a-select-option v-for="model in data.modelList" :value="model.modelName">{{ model.name }}

                    </a-select-option>
                </a-select>
                <div class="menu_name">记录</div>
                <div class="record_box">
                    <div v-for="(record,index) in data.recordList" :class="{choose:data.nowRecordId===record.id}"
                    >
                        <div @click="load_record(record)">{{ record.title }}</div>
                        <div class="close_icon"><img src="@/assets/icon/close.png" @click="delRecord(index)" alt/></div>

                    </div>
                </div>
                <div class="lp_btn_box">
                    <a-button type="primary" @click="toggleAdd(true)" size="middle">新增会话</a-button>
                    <a-button type="primary" @click="openSetting" size="middle">更多设置</a-button>
                </div>
            </div>
            <div class="lp_arrow">👉</div>
        </div>
        <addChat v-if="data.addChat" @save="saveNewChat" @toggle="toggleAdd"/>
    </div>
</template>
<script setup>
import {nextTick, onMounted, ref, createVNode, watch} from 'vue';
import {SendOutlined, DeleteOutlined, ExclamationCircleOutlined} from '@ant-design/icons-vue';
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
import {message} from 'ant-design-vue';
import AddChat from "../components/addChat.vue";
import {Modal} from 'ant-design-vue';

let data = ref({
    sending: false,
    msgList: [],
    recordList: [
        {
            id: "1",
            title: "默认会话",
        },
        {
            id: "2",
            title: "特殊会话",
        }
    ],
    modelList: [
        {
            name: "千问2 1.5B",
            modelName: "qwen2-1.5b-instruct"
        },
        {
            name: "千问2 72B",
            modelName: "qwen2-72b-instruct"
        }
    ],
    inputText: "",
    nowRecordId: '1',
    nowModelName: 'qwen2-1.5b-instruct',
    addChat: false
});
//监听页面数据变化，逻辑简单，不考虑性能
watch(data, (newVal, oldVal) => {
    //保存状态
    updateState()
    //nowRecordId改变时，切换列表
    message.warn(newVal.nowRecordId, 1);
    message.warn(oldVal.nowRecordId, 1);
    if(newVal.nowRecordId!==oldVal.nowRecordId){
        message.warn(newVal.nowRecordId, 1);
    }
}, {deep: true});

onMounted(() => {
    //加载会话
    localStorage.setItem("time", new Date().getTime().toString());
    let state = localStorage.getItem("state");
    if (state == null) {
        updateState()
    } else {
        data.value = JSON.parse(state);
    }
    toBottom();
});
const updateState = () => {
    localStorage.setItem("state", JSON.stringify(data.value));
}

const clearChat = () => {

    Modal.confirm({
        title: '确认',
        icon: createVNode(ExclamationCircleOutlined),
        content: '确认清除本会话的消息记录吗',
        okText: '确认',
        cancelText: '取消',
        onOk: () => {
            data.value.msgList = []
            localStorage.removeItem("record:" + data.value.nowRecordId);
        }
    });

}
const delRecord = (index) => {
    let record = data.value.recordList[index];
    if (data.value.recordList.length === 1) {
        message.warn('至少保留一个会话哦', 1);
        return;
    }
    localStorage.removeItem("record:" + record.id);
    data.value.recordList.splice(index, 1);
}
const toggleAdd = (state) => {
    data.value.addChat = state
}
const openSetting = () => {
    invoke("setting")
}
const saveNewChat = (chat) => {
    console.log(data.value)
    let record = {
        id: new Date().getTime(),
        title: chat.name,
        prompt: chat.prompt
    };
    data.value.recordList.push(record)
    load_record(record);
    data.value.addChat = false;
    if (chat.prompt) {
        data.value.msgList.unshift({
            id: 999,
            role: "system",
            avatar: "https://res.ztion.cn/imgs/cat.png",
            content: chat.prompt,
        });
    }
}
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
            model_name: data.value.nowModelName,
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
    data.value.nowRecordId = record.id
    let msgRecord = localStorage.getItem("record:" + record.id);
    if (msgRecord != null) {
        let msgList = JSON.parse(msgRecord);
        if (msgList.length > 0) {
            data.value.msgList = msgList;
        }
    } else {
        data.value.msgList = []
    }
    toBottom();
    message.success('已成功加载历史消息', 1);
}

const quickSend = e => {
    if (e.shiftKey === true && e.key === "Enter") {
        sendMsg();
    }
};

listen("msg_chunk", e => {
    let msg = e.payload;
    merge_msg_chunk(msg.msg_id, msg.chunk_content);
    if (msg.over) {
        localStorage.setItem("record:" + data.value.nowRecordId, JSON.stringify(data.value.msgList));
    }
});
listen("clear_data", e => {
    window.location.reload();
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
        left: -220px;
        z-index: 100;
        transition: all 200ms ease-in;
        display: flex;
        align-items: center;
        user-select: none;


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
                height: 300px;
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
                    display: flex;

                    &:hover .close_icon {
                        visibility: visible;
                    }

                    .close_icon {
                        visibility: hidden;
                    }

                    > div:nth-child(1) {
                        width: 100%;
                        height: 40px;
                        line-height: 40px;
                        text-indent: 20px;
                    }

                    > div:nth-child(2) {
                        width: 20%;
                        height: 40px;
                        align-items: center;
                        justify-content: center;
                        display: flex;

                        img {
                            width: 20px;
                        }
                    }

                    &:hover {
                        cursor: pointer;
                        background: #7873f1;
                    }
                }
            }


            .lp_btn_box {
                width: 100%;
                height: 50px;
                display: flex;
                margin-top: 10px;
                align-items: center;
                justify-content: space-around;
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
            background: rgba(255, 255, 244, 1);
            opacity: 0.8;
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