FROM python:latest

RUN apt-get update
RUN python3 -m pip install --upgrade pip

RUN mkdir -p /opt/server/logs
COPY requirements.txt /opt/server/requirements.txt

WORKDIR /opt/server
RUN pip install -r requirements.txt

COPY ./ /opt/server/
COPY ./entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
CMD [ "/entrypoint.sh" ]
