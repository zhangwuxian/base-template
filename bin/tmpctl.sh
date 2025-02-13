#!/bin/sh

# 初始化工作目录和变量
workdir=$(cd $(dirname $0); pwd)
logs_dir="${workdir}/../logs"
libs_dir="${workdir}/../libs"
config_dir="${workdir}/../config"
DEFAULT_MOD="scale-dispatcher"

# 确保日志目录存在
mkdir -p "${logs_dir}"

# 参数验证函数
validate_params() {
    mod=$1
    action=$2

    if [ -z "$mod" ]; then
        echo "No component specified, using default: ${DEFAULT_MOD}"
        mod=$DEFAULT_MOD
    fi

    if [ -z "$action" ]; then
        echo "Please make sure the position variable is start, stop, or restart."
        return 1
    fi

    case "$action" in
        start|stop|restart) ;;
        *)
            echo "Invalid action. Please use start, stop, or restart."
            return 1
            ;;
    esac
}

# 检查进程状态
check_process() {
    mod=$1
    ps -ef | grep "${mod}" | grep conf | grep -v grep | wc -l
}

# 获取进程ID
get_pid() {
    mod=$1
    ps -ef | grep "${mod}" | grep conf | grep -v grep | awk '{print $2}'
}

# 启动服务
start_service() {
    mod=$1
    conf=$2

    # 如果配置文件未指定，使用默认配置
    if [ -z "$conf" ]; then
        conf="${config_dir}/${mod}.toml"
    fi

    # 检查服务是否已经在运行
    if [ "$(check_process "${mod}")" -gt 0 ]; then
        echo "${mod} is already running."
        return 1
    fi

    echo "config: $conf"
    echo "${mod} is starting...."
    nohup "${libs_dir}/${mod}" --conf="$conf" >> "${logs_dir}/${mod}-nohup.log" 2>&1 &

    # 等待服务启动
    sleep 1

    if [ "$(check_process "${mod}")" -gt 0 ]; then
        echo "${mod} started successfully."
        return 0
    else
        echo "${mod} failed to start."
        return 1
    fi
}

# 停止服务
stop_service() {
    mod=$1
    pid=$(get_pid "${mod}")

    if [ -n "$pid" ]; then
        echo "Currently running process number: $pid"
        kill "$pid"

        # 等待进程结束
        i=0
        while [ $i -lt 10 ]; do
            if [ "$(check_process "${mod}")" -eq 0 ]; then
                echo "${mod} stopped successfully."
                return 0
            fi
            i=$((i + 1))
            sleep 1
        done

        # 如果进程仍然存在，强制终止
        echo "Force killing ${mod}..."
        kill -9 "$pid" 2>/dev/null
        echo "${mod} force stopped."
        return 0
    else
        echo "No running process found for ${mod}."
        return 1
    fi
}

# 重启服务
restart_service() {
    mod=$1
    conf=$2

    echo "Restarting ${mod}..."
    stop_service "${mod}"
    sleep 2  # 等待服务完全停止
    start_service "${mod}" "${conf}"
}

# 主函数
main() {
    action=$1
    mod=${2:-sample}
    conf=$3

    if ! validate_params "$mod" "$action"; then
        exit 1
    fi

    case "$action" in
        start)
            start_service "$mod" "$conf"
            ;;
        stop)
            stop_service "$mod"
            ;;
        restart)
            restart_service "$mod" "$conf"
            ;;
        *)
            echo "Invalid action: $action"
            exit 1
            ;;
    esac
}

# 执行主函数
main "$@"
