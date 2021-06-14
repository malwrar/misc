"""Attempt at monocamera slam."""

from pathlib import Path
from typing import Tuple

import cv2
import numpy as np

window_name = 'facedetector'
cv2.namedWindow(window_name)
cv2.createTrackbar('Threshold 1', window_name, 0, 255, lambda _: None)
cv2.createTrackbar('Threshold 2', window_name, 0, 255, lambda _: None)

#camera = cv2.VideoCapture("v4l2src device=/dev/video0 ! video/x-raw,format=YUY2,width=640,height=480,framerate=30/1 ! videoconvert ! video/x-raw, format=BGR ! appsink drop=1 ", cv2.CAP_GSTREAMER)
camera = cv2.VideoCapture(2)
camera.set(cv2.CAP_PROP_FRAME_WIDTH, 640)
camera.set(cv2.CAP_PROP_FRAME_HEIGHT, 480)

while True:
    _, frame = camera.read()
    frame = cv2.flip(frame, 1)

    t1 = cv2.getTrackbarPos('Threshold 1', window_name)
    t2 = cv2.getTrackbarPos('Threshold 2', window_name)
    gb = cv2.GaussianBlur(frame, (5, 5), 0)
    can = cv2.Canny(gb, t1, t2)

    cv2.imshow('canny', can)

    frame[np.where(can)] = 255
    cv2.imshow('WebCam', frame)
    if cv2.waitKey(1) == ord('q'):
        break

camera.release()
cv2.destroyAllWindows()
