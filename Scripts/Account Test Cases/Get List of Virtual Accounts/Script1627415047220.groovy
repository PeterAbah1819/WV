import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

//Send the request and save it the http response in variable response
response = WS.sendRequest(findTestObject('Create Account/Get list of virtual accounts'))

//Verify that the response gotten and saved in response is 200
WS.verifyResponseStatusCode(response, 200)

//Verify that the response has the value a messge in the body stating the process was completed successfully.
WS.verifyElementPropertyValue(response, 'message', 'The process was completed successfully')

//Verify that the response has the value 1 for number of pages info in the body
WS.verifyElementPropertyValue(response, 'data.page_info.total', '1')

//Verify that the response has the value 1 for total pages in the body
WS.verifyElementPropertyValue(response, 'data.page_info.total_pages', '1')

//Verify that the response has the value jones' email in the body
WS.verifyElementPropertyValue(response, 'data.vnubans[0].account_email', 'jones_adelaide@mail.com')

