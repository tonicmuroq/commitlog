FROM python
RUN pip install pip --upgrade
ADD commitlog /commitlog
ADD requirements.txt /commitlog/requirements.txt
WORKDIR /commitlog
RUN pip install -r ./requirements.txt
EXPOSE 5000
CMD ["gunicorn", "app:app", "-b", "0.0.0.0:5000", "-w", "4"]
