import json

import matplotlib.pyplot as plt
import paho.mqtt.client as mqtt
from numpy import sin, cos

x = []
y = []

length = 0.8


def on_message(client, userdata, message):
    # print(str(message.payload))

    if message.topic == "CTRL/out2d":
        data = json.loads(message.payload)
        x.append(data["x"]["x"])
        y.append(data["v"]["x"])
        return

    if message.topic == "CTRL/end2d":
        print("plotting pendulum...")
        x1 = length * sin(x)
        x2 = -length * cos(x)

        plt.axis('equal')
        plt.plot(x1, x2)
        plt.show()

        x.clear()
        y.clear()
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
