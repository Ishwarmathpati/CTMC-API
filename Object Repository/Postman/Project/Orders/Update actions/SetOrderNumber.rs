<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetOrderNumber</name>
   <tag></tag>
   <elementGuidId>5bc9530e-fa5f-47f3-b905-e89948bc2ebf</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>2tixOwe5b29Ug-hr8HclT_qtmx4pZ4uo</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${GlobalVariable.order_version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setOrderNumber\&quot;,\n            \&quot;orderNumber\&quot; : \&quot;003\&quot;\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a5661649-d74e-43a9-bd2b-75d22661ef7e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer 2tixOwe5b29Ug-hr8HclT_qtmx4pZ4uo</value>
      <webElementGuid>ca779cbc-b0cf-42bb-bb56-5c2c7e900acd</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.host}/${GlobalVariable.project_key}/orders/${GlobalVariable.order_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>15ab59f4-b599-4c41-bdbd-15b139b182f6</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project_key</defaultValue>
      <description></description>
      <id>a373c84f-861b-4bdf-a9d4-88046f13a557</id>
      <masked>false</masked>
      <name>project_key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order_id</defaultValue>
      <description></description>
      <id>a8396e23-af87-43ad-bd54-ee50dde02371</id>
      <masked>false</masked>
      <name>order_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order_version</defaultValue>
      <description></description>
      <id>489ab25c-7973-4e82-b561-a107a8de6347</id>
      <masked>false</masked>
      <name>order_version</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
