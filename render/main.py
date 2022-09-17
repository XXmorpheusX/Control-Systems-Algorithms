import paho.mqtt.client as mqtt
import json
import time
from mpl_toolkits import mplot3d
import numpy as np
import matplotlib.pyplot as plt

x = []
y = []
z = []


def on_message(client, userdata, message):
    if message.topic == "CTRL/end":
        print("hello")
        fig = plt.figure()
        ax = plt.axes(projection="3d")
        ax.plot3D(x, y, z, 'gray')
        plt.show()
        time.sleep(5)
        return

    data = json.loads(message.payload)
    x.append(data["x"]["x"])
    y.append(data["x"]["y"])
    z.append(data["x"]["z"])
    print(x)
    print("-----------------")


if __name__ == "__main__":
    print("Setting up mqtt connection")
    mqttc = mqtt.Client()

    mqttc.connect("localhost")
    print("Successfully connected to mqtt broker")

    print("Subscribing to output topics")
    mqttc.subscribe("CTRL/out", 2)
    mqttc.subscribe("CTRL/end", 2)

    print("Setting Callbacks...")
    mqttc.on_message = on_message

    print("Looping...")
    mqttc.loop_forever()
