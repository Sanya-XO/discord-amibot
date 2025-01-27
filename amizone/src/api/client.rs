use super::{types::{*, go_amizone::server::proto::v1::{ExamResultRecord, OverallResult}}, user::credentials::UserMetaData};
use go_amizone::server::proto::v1::{
    ClassScheduleRequest, DeregisterWifiMacRequest, EmptyMessage, FillFacultyFeedbackRequest,
    RegisterWifiMacRequest, SemesterRef,
};
use tonic::Request;

#[derive(Clone)]
pub struct UserClient {
    metadata: UserMetaData,
    connection: AmizoneConnection,
}

impl UserClient {
    pub fn new(metadata: UserMetaData, connection: AmizoneConnection) -> Self {
        Self {
            metadata,
            connection,
        }
    }

    fn prepare_request<M>(&self, message: M) -> Request<M> {
        let mut request = Request::new(message);
        request
            .metadata_mut()
            .insert("authorization", self.metadata.clone());
        request
    }

    pub async fn get_attendance(&mut self) -> Result<Vec<AttendanceRecord>> {
        let request = self.prepare_request(EmptyMessage {});

        let mut amizone = self.connection.lock().await;
        let response = amizone.get_attendance(request).await?.into_inner();
        drop(amizone);

        Ok(response.records)
    }

    pub async fn get_exam_schedule(&mut self) -> Result<(String, Vec<ScheduledExam>)> {
        let request = self.prepare_request(EmptyMessage {});

        let mut amizone = self.connection.lock().await;
        let response = amizone.get_exam_schedule(request).await?.into_inner();
        drop(amizone);

        Ok((response.title, response.exams))
    }
    

    pub async fn get_exam_result(&mut self, num: usize) -> Result<(Vec<ExamResultRecord>, Vec<OverallResult>)> {
        let request = self.prepare_request(SemesterRef {
            semester_ref: num.to_string(),
        });

        let mut amizone = self.connection.lock().await;
        let response = amizone.get_exam_result(request).await?.into_inner();
        drop(amizone);

        Ok((response.course_wise, response.overall))
    }

    pub async fn get_current_exam_result(&mut self) -> Result<(Vec<ExamResultRecord>, Vec<OverallResult>)> {
        let request = self.prepare_request(EmptyMessage {});

        let mut amizone = self.connection.lock().await;
        let response = amizone.get_current_exam_result(request).await?.into_inner();
        drop(amizone);

        Ok((response.course_wise, response.overall))
    }
    

    pub async fn get_semesters(&mut self) -> Result<Vec<Semester>> {
        let request = self.prepare_request(EmptyMessage {});

        let mut amizone = self.connection.lock().await;
        let response = amizone.get_semesters(request).await?.into_inner();
        drop(amizone);

        Ok(response.semesters)
    }

    pub async fn get_current_courses(&mut self) -> Result<Vec<Course>> {
        let request = self.prepare_request(EmptyMessage {});

        let mut amizone = self.connection.lock().await;
        let response = amizone.get_current_courses(request).await?.into_inner();
        drop(amizone);

        Ok(response.courses)
    }

    pub async fn get_user_profile(&mut self) -> Result<AmizoneProfile> {
        let request = self.prepare_request(EmptyMessage {});

        let mut amizone = self.connection.lock().await;
        let response = amizone.get_user_profile(request).await?.into_inner();
        drop(amizone);

        Ok(response)
    }

    pub async fn get_wifi_mac_info(&mut self) -> Result<WifiMacInfo> {
        let request = self.prepare_request(EmptyMessage {});

        let mut amizone = self.connection.lock().await;
        let response = amizone.get_wifi_mac_info(request).await?.into_inner();
        drop(amizone);

        Ok(response)
    }

    pub async fn get_courses(&mut self, num: usize) -> Result<Vec<Course>> {
        let request = self.prepare_request(SemesterRef {
            semester_ref: num.to_string(),
        });

        let mut amizone = self.connection.lock().await;
        let response = amizone.get_courses(request).await?.into_inner();
        drop(amizone);

        Ok(response.courses)
    }

    pub async fn register_wifi_mac(&mut self, addr: impl ToString) -> Result<()> {
        let request = self.prepare_request(RegisterWifiMacRequest {
            address: addr.to_string(),
            override_limit: true,
        });

        let mut amizone = self.connection.lock().await;
        amizone.register_wifi_mac(request).await?;
        drop(amizone);

        Ok(())
    }

    pub async fn deregister_wifi_mac(&mut self, addr: impl ToString) -> Result<()> {
        let request = self.prepare_request(DeregisterWifiMacRequest {
            address: addr.to_string(),
        });

        let mut amizone = self.connection.lock().await;
        amizone.deregister_wifi_mac(request).await?;
        drop(amizone);

        Ok(())
    }

    pub async fn fill_faculty_feedback(
        &mut self,
        rating: i32,
        query_rating: i32,
        comment: impl ToString,
    ) -> Result<i32> {
        let request = self.prepare_request(FillFacultyFeedbackRequest {
            rating,
            query_rating,
            comment: comment.to_string(),
        });

        let mut amizone = self.connection.lock().await;
        let filled = amizone.fill_faculty_feedback(request).await?.into_inner();
        drop(amizone);

        Ok(filled.filled_for)
    }

    pub async fn get_class_schedule(&mut self, date: Date) -> Result<Vec<ScheduledClass>> {
        let request = self.prepare_request(ClassScheduleRequest { date: Some(date) });

        let mut amizone = self.connection.lock().await;
        let response = amizone.get_class_schedule(request).await?.into_inner();
        drop(amizone);
        Ok(response.classes)
    }
}
