"""Simple webcam face detection using harr cascades."""

from pathlib import Path
from typing import Tuple

import cv2

# Tune following https://stackoverflow.com/a/20805153/14928812
SCALE_FACTOR=1.1
MIN_NEIGHBORS=4

class Color:
    def __init__(self, r: float, g: float, b: float, a: float=1.0):
        assert r >= 0.0 and r <= 1.0
        assert g >= 0.0 and g <= 1.0
        assert b >= 0.0 and b <= 1.0
        assert a >= 0.0 and a <= 1.0

        self.r = r
        self.g = g
        self.b = b
        self.a = b

    def print(self, msg: str):
        """Print a message using the 24-bit color ANSI escape code extension.

        More info: https://en.wikipedia.org/wiki/ANSI_escape_code#24-bit
        """
        print("" + msg + "")

    def to_tuple(self) -> Tuple[float, float, float]:
        return (self.r * 255, self.g * 255, self.b * 255,)
        

class Detector:
    def __init__(self, color: Color):
        self.color = color

    def process_frame(self, frame):
        raise NotImplementedError

class CascadeDetector(Detector):
    def __init__(self, color: Color, classifier_filename: str,
            scale_factor: float, min_neighbors: int):
        super().__init__(color)

        self.scale_factor = scale_factor
        self.min_neighbors = min_neighbors
        self.classifier = cv2.CascadeClassifier(classifier_filename)

    def process_frame(self, frame):
        frame_grayscale = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
        faces = self.classifier.detectMultiScale(frame_grayscale,
                self.scale_factor, self.min_neighbors)
        return [ (x, y, x+w, y+h) for (x, y, w, h) in faces ]

#class CaffeDetector(Detector):
#    def __init__(self, color: Color, model_path: str, prototxt_path: str):
#        super().__init__(color)
#
#        self.net = cv2.dnn.readNetFromCafe(prototxt_path, model_path)
#
#    def process_frame(self, frame):
#        pass

assets_base = Path(__file__).parent.absolute() / 'assets'

camera = cv2.VideoCapture("v4l2src device=/dev/video0 ! video/x-raw,format=YUY2,width=640,height=480,framerate=30/1 ! videoconvert ! video/x-raw, format=BGR ! appsink drop=1 ", cv2.CAP_GSTREAMER)

detectors = [
    CascadeDetector(Color(1.0, 0, 0),
            str(assets_base / 'models' / 'haarcascade_frontalface_default.xml'),
            SCALE_FACTOR, MIN_NEIGHBORS),
    CascadeDetector(Color(0, 1.0, 0),
            str(assets_base / 'models' / 'haarcascade_frontalface_alt.xml'),
            SCALE_FACTOR, MIN_NEIGHBORS),
    CascadeDetector(Color(0, 0, 1.0),
            str(assets_base / 'models' / 'haarcascade_frontalface_alt2.xml'),
            SCALE_FACTOR, MIN_NEIGHBORS),
    CascadeDetector(Color(1.0, 1.0, 0),
            str(assets_base / 'models' / 'haarcascade_frontalface_alt_tree.xml'),
            SCALE_FACTOR, MIN_NEIGHBORS),
]

while True:
    _, frame = camera.read()

    for detector in detectors:
        faces = detector.process_frame(frame)
        for (x_start, y_start, x_end, y_end) in faces:
            cv2.rectangle(frame, (x_start, y_start), (x_end, y_end),
                    detector.color.to_tuple(), 2)
            
    cv2.imshow('frame', frame)
    if cv2.waitKey(1) == ord('q'):
        break

camera.release()
cv2.destroyAllWindows()
