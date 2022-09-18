import json

import matplotlib.pyplot as plt
import numpy as np
import paho.mqtt.client as mqtt

x1 = []
x2 = []
v1 = []
v2 = []


def on_message(client, userdata, message):
    # print(str(message.payload))

    if message.topic == "CTRL/out2d":
        data = json.loads(message.payload)
        x1.append(data["x"]["x"])
        x2.append(data["x"]["y"])
        v1.append(data["v"]["x"])
        v2.append(data["v"]["y"])
        return

    if message.topic == "CTRL/end2d":
        print("plotting all x variable signals...")
        t = np.linspace(0, 1, len(x1))

        plt.style.use('dark_background')
        fig, axs = plt.subplots(2, 2)

        axs[0, 0].plot(t, x1, color='m')
        axs[0, 0].set_title('x1', fontsize=10)

        axs[0, 1].plot(t, x2, color='m')
        axs[0, 1].set_title('x2', fontsize=10)

        axs[1, 0].plot(t, v1, color='y')
        axs[1, 0].set_title('v1', fontsize=10)

        axs[1, 1].plot(t, v2, color='y')
        axs[1, 1].set_title('v2', fontsize=10)

        plt.show()

        x1.clear()
        x2.clear()
        v1.clear()
        v2.clear()
        return


if __name__ == "__main__":
    print("Setting up mqtt connection")
    mqttc = mqtt.Client()

    mqttc.connect("localhost")
    print("Successfully connected to mqtt broker")

    print("Subscribing to output topics")
    mqttc.subscribe("CTRL/out2d", 2)
    mqttc.subscribe("CTRL/end2d", 2)

    print("Setting Callbacks...")
    mqttc.on_message = on_message

    print("Looping...")
    mqttc.loop_forever()
